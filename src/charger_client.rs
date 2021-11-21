use std::time::Instant;

use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use log::{info, warn};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::logs;
use crate::messages::*;
use crate::server;
use crate::server::MessageFromChargeStation;

pub struct DefaultResponses {
    pub authorize: responses::AuthorizeResponse,
    pub data_transfer: responses::DataTransferResponse,
    pub sign_certificate: responses::SignCertificateResponse,
    pub start_transaction: responses::StartTransactionResponse,
    pub stop_transaction: responses::StopTransactionResponse,
}

pub struct ChargeStationWebSocketSession {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT)
    /// otherwise the connection will b e dropped
    pub hb: Instant,
    pub name: String,
    pub db_connection: PooledConnection<SqliteConnectionManager>,
    pub address: Addr<server::OcppServer>,
    pub default_responses: DefaultResponses,
}

impl Actor for ChargeStationWebSocketSession {
    type Context = ws::WebsocketContext<Self>;
    /// Method is called on actor start. We register websocket session with charge point
    fn started(&mut self, ctx: &mut Self::Context) {
        // we start heartbeat process on session start
        self.hb(ctx);

        // register self in ocpp server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of ChargePointWebSocketSession, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.address
            .send(server::ConnectCharger {
                addr: addr.recipient(),
                serial_id: self.name.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.name = res,
                    // something is wrong with the ocpp server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.address.do_send(server::DisconnectCharger {
            serial_id: self.name.clone(),
        });
        Running::Stop
    }
}

fn strip_quotes(input: &String) -> &str {
    input
        .strip_prefix("\"")
        .unwrap()
        .strip_suffix("\"")
        .unwrap()
}

impl ChargeStationWebSocketSession {
    /// helper method that sends ping to client every second.
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                info!("Websocket Client heartbeat failed, disconnecting!");
                act.address.do_send(server::DisconnectCharger {
                    serial_id: act.name.clone(),
                });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Handler<server::MessageToChargeStation> for ChargeStationWebSocketSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: server::MessageToChargeStation,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(authorize_response) = msg.authorize {
            self.default_responses.authorize = authorize_response;
        }
        if let Some(data_transfer_response) = msg.data_transfer {
            self.default_responses.data_transfer = data_transfer_response;
        }
        if let Some(sign_certificate_response) = msg.sign_certificate {
            self.default_responses.sign_certificate = sign_certificate_response;
        }
        if let Some(start_transaction_response) = msg.start_transaction {
            self.default_responses.start_transaction = start_transaction_response;
        }
        if let Some(stop_transaction_response) = msg.stop_transaction {
            self.default_responses.stop_transaction = stop_transaction_response;
        }
        if let Some(message) = msg.message {
            ctx.text(message)
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChargeStationWebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(message) => match message {
                ws::Message::Ping(msg) => {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                }
                ws::Message::Pong(_) => {
                    self.hb = Instant::now();
                }
                ws::Message::Text(text) => {
                    logs::add_log(
                        &self.db_connection,
                        &self.name,
                        None,
                        format!("incoming message: {}", text.clone()),
                    );
                    if let Ok(unpacked) = unpack_ocpp_message(&text) {
                        let message_type_id: u8 =
                            unpacked.get("MessageTypeId").unwrap().parse().unwrap();
                        match message_type_id {
                            2 => {
                                let action: &str =
                                    &unpacked.get("Action").unwrap().as_str().replace("\"", "");
                                match action {
                                    "Authorize" => {
                                        if let Err(e) = serde_json::from_str(
                                            &unpacked.get("Payload").unwrap(),
                                        )
                                            as Result<requests::AuthorizeRequest, serde_json::Error>
                                        {
                                            let call_error_result = wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::FormatViolation,
                                                &format!("{:#?}", e),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                Some(String::from("error")),
                                                format!(
                                                    "outgoing response: {}",
                                                    call_error_result.clone()
                                                ),
                                            );
                                            ctx.text(call_error_result)
                                        } else {
                                            let call_result = wrap_call_result(
                                                unpacked.get("MessageId").unwrap(),
                                                serde_json::to_string(
                                                    &self.default_responses.authorize,
                                                )
                                                    .unwrap(),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                None,
                                                format!(
                                                    "outgoing response: {}",
                                                    call_result.clone()
                                                ),
                                            );
                                            ctx.text(call_result);
                                        }
                                    }
                                    "BootNotification" => {
                                        let call_result = boot_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "DataTransfer" => {
                                        if let Err(e) =
                                            serde_json::from_str(&unpacked.get("Payload").unwrap())
                                                as Result<
                                                    requests::DataTransferRequest,
                                                    serde_json::Error,
                                                >
                                        {
                                            let call_error_result = wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::FormatViolation,
                                                &format!("{:#?}", e),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                Some(String::from("error")),
                                                format!(
                                                    "outgoing response: {}",
                                                    call_error_result.clone()
                                                ),
                                            );
                                            ctx.text(call_error_result);
                                        } else {
                                            let call_result = wrap_call_result(
                                                unpacked.get("MessageId").unwrap(),
                                                serde_json::to_string(
                                                    &self.default_responses.data_transfer,
                                                )
                                                .unwrap(),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                None,
                                                format!(
                                                    "outgoing response: {}",
                                                    call_result.clone()
                                                ),
                                            );
                                            ctx.text(call_result);
                                        }
                                    }
                                    "DiagnosticsStatusNotification" => {
                                        let call_result = diagnostics_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "FirmwareStatusNotification" => {
                                        let call_result = firmware_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result)
                                    }
                                    "Heartbeat" => {
                                        let call_result =
                                            heartbeat_response(unpacked.get("MessageId").unwrap());
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result)
                                    }
                                    "MeterValues" => {
                                        let call_result = meter_values_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "StartTransaction" => {
                                        if let Err(e) =
                                            serde_json::from_str(&unpacked.get("Payload").unwrap())
                                                as Result<
                                                    requests::StartTransactionRequest,
                                                    serde_json::Error,
                                                >
                                        {
                                            let call_error_result = wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::FormatViolation,
                                                &format!("{:#?}", e),
                                            );

                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                Some(String::from("error")),
                                                format!(
                                                    "outgoing response: {}",
                                                    call_error_result.clone()
                                                ),
                                            );
                                            ctx.text(call_error_result);
                                        } else {
                                            let call_result = wrap_call_result(
                                                unpacked.get("MessageId").unwrap(),
                                                serde_json::to_string(
                                                    &self.default_responses.start_transaction,
                                                )
                                                .unwrap(),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                None,
                                                format!(
                                                    "outgoing response: {}",
                                                    call_result.clone()
                                                ),
                                            );
                                            ctx.text(call_result);
                                        }
                                    }
                                    "StatusNotification" => {
                                        let call_result = status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "StopTransaction" => {
                                        if let Err(e) =
                                            serde_json::from_str(&unpacked.get("Payload").unwrap())
                                                as Result<
                                                    requests::StopTransactionRequest,
                                                    serde_json::Error,
                                                >
                                        {
                                            let call_error_result = wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::FormatViolation,
                                                &format!("{:#?}", e),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                Some(String::from("error")),
                                                format!(
                                                    "outgoing response: {}",
                                                    call_error_result.clone()
                                                ),
                                            );
                                            ctx.text(call_error_result.clone());
                                        } else {
                                            let call_result = wrap_call_result(
                                                unpacked.get("MessageId").unwrap(),
                                                serde_json::to_string(
                                                    &self.default_responses.stop_transaction,
                                                )
                                                .unwrap(),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                None,
                                                format!(
                                                    "outgoing response: {}",
                                                    call_result.clone()
                                                ),
                                            );
                                            ctx.text(call_result);
                                        }
                                    }
                                    "LogStatusNotification" => {
                                        let call_result = log_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "SecurityEventNotification" => {
                                        let call_result = security_event_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            Some(String::from("error")),
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }
                                    "SignCertificate" => {
                                        if let Err(e) =
                                            serde_json::from_str(&unpacked.get("Payload").unwrap())
                                                as Result<
                                                    requests::SignCertificateRequest,
                                                    serde_json::Error,
                                                >
                                        {
                                            let call_error_result = wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::FormatViolation,
                                                &format!("{:#?}", e),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                Some(String::from("error")),
                                                format!(
                                                    "outgoing response: {}",
                                                    call_error_result.clone()
                                                ),
                                            );
                                            ctx.text(call_error_result.clone());
                                        } else {
                                            let call_result = wrap_call_result(
                                                unpacked.get("MessageId").unwrap(),
                                                serde_json::to_string(
                                                    &self.default_responses.stop_transaction,
                                                )
                                                .unwrap(),
                                            );
                                            logs::add_log(
                                                &self.db_connection,
                                                &self.name,
                                                None,
                                                format!(
                                                    "outgoing response: {}",
                                                    call_result.clone()
                                                ),
                                            );
                                            ctx.text(call_result.clone());
                                        }
                                    }
                                    "SignedFirmwareStatusNotification" => {
                                        let call_result =
                                            signed_firmware_status_notification_response(
                                                unpacked.get("MessageId").unwrap(),
                                                unpacked.get("Payload").unwrap(),
                                            );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            None,
                                            format!("outgoing response: {}", call_result.clone()),
                                        );
                                        ctx.text(call_result);
                                    }

                                    _ => {
                                        let call_error_result = wrap_call_error_result(
                                            unpacked.get("MessageId").unwrap(),
                                            ErrorCode::NotImplemented,
                                            &String::from(
                                                "\"Not all messages are implemented yet. \
                                                    Ocpp server is still in development\"",
                                            ),
                                        );
                                        logs::add_log(
                                            &self.db_connection,
                                            &self.name,
                                            Some(String::from("error")),
                                            format!(
                                                "outgoing response: {}",
                                                call_error_result.clone()
                                            ),
                                        );
                                        ctx.text(call_error_result.clone());
                                    }
                                }
                            }
                            3 => {
                                let message_id_option = unpacked.get("MessageId");
                                let payload_option = unpacked.get("Payload");
                                if message_id_option.is_some() && payload_option.is_some() {
                                    let call_result = CallResult {
                                        unique_id: message_id_option.unwrap().clone(),
                                        payload: serde_json::from_str(
                                            payload_option.unwrap().clone().as_str(),
                                        )
                                        .unwrap(),
                                    };
                                    self.address.do_send(MessageFromChargeStation {
                                        charger_id: self.name.clone(),
                                        call: None,
                                        call_result: Some(call_result),
                                        call_error: None,
                                    });
                                }
                            }
                            4 => {
                                let message_id_option = unpacked.get("MessageId");
                                let error_code_option = unpacked.get("ErrorCode");
                                let error_description_option = unpacked.get("ErrorDescription");
                                let error_details_option = unpacked.get("ErrorDetails");
                                if message_id_option.is_some()
                                    && error_code_option.is_some()
                                    && error_description_option.is_some()
                                    && error_details_option.is_some()
                                {
                                    let call_error = CallError {
                                        unique_id: strip_quotes(&message_id_option.unwrap())
                                            .parse()
                                            .unwrap(),
                                        error_code: strip_quotes(&error_code_option.unwrap())
                                            .parse()
                                            .unwrap(),
                                        error_description: strip_quotes(
                                            &error_description_option.unwrap(),
                                        )
                                        .parse()
                                        .unwrap(),
                                        error_details: error_details_option.unwrap().clone(),
                                    };
                                    self.address.do_send(MessageFromChargeStation {
                                        charger_id: self.name.clone(),
                                        call: None,
                                        call_result: None,
                                        call_error: Some(call_error),
                                    })
                                }
                            }
                            _ => {}
                        }
                    }
                }
                ws::Message::Binary(_) => warn!("Unexpected binary"),
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => ctx.stop(),
            },
        };
    }
}
