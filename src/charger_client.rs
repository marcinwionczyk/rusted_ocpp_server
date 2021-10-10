use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use crate::messages::*;
use crate::server;
use log::{info, warn, error};
use actix_web_actors::ws::ProtocolError;
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
        self.address.send(server::ConnectCharger {
            addr: addr.recipient(),
            serial_id: self.name.clone(),
        }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.name = res,
                    // something is wrong with the ocpp server
                    _ => ctx.stop(),
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.address.do_send(server::DisconnectCharger { serial_id: self.name.clone() });
        Running::Stop
    }
}

fn strip_quotes(input: &String) -> &str {
    input.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap()
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
                act.address.do_send(server::DisconnectCharger { serial_id: act.name.clone() });
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

    fn handle(&mut self, msg: server::MessageToChargeStation, ctx: &mut Self::Context) -> Self::Result {
        if msg.authorize.is_some() {
            self.default_responses.authorize = msg.authorize.unwrap();
        }
        if msg.data_transfer.is_some() {
            self.default_responses.data_transfer = msg.data_transfer.unwrap();
        }
        if msg.sign_certificate.is_some() {
            self.default_responses.sign_certificate = msg.sign_certificate.unwrap();
        }
        if msg.start_transaction.is_some() {
            self.default_responses.start_transaction = msg.start_transaction.unwrap();
        }
        if msg.stop_transaction.is_some() {
            self.default_responses.stop_transaction = msg.stop_transaction.unwrap();
        }
        if msg.message.is_some() {
            ctx.text(msg.message.unwrap())
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChargeStationWebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                info!("{}: incoming message: {:?}", self.name, text);
                match unpack_ocpp_message(&text) {
                    Ok(unpacked) => {
                        let message_type_id: u8 = unpacked.get("MessageTypeId").unwrap().parse()
                            .unwrap();
                        match message_type_id {
                            2 => {
                                let action: &str = &unpacked.get("Action").unwrap().as_str()
                                    .replace("\"", "");
                                match action {
                                    "Authorize" => {
                                        match serde_json::from_str(&unpacked.get("Payload").unwrap()) as Result<requests::AuthorizeRequest, serde_json::Error> {
                                            Ok(_) => {
                                                ctx.text(wrap_call_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    serde_json::to_string(&self.default_responses.authorize).unwrap()))
                                            }
                                            Err(e) => {
                                                ctx.text(wrap_call_error_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    ErrorCode::FormatViolation,
                                                    &format!("{:#?}", e)))
                                            }
                                        }
                                    }
                                    "BootNotification" => {
                                        let response = boot_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }
                                    "DataTransfer" => {
                                        match serde_json::from_str(&unpacked.get("Payload").unwrap()) as Result<requests::DataTransferRequest, serde_json::Error> {
                                            Ok(_) => {
                                                ctx.text(wrap_call_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    serde_json::to_string(&self.default_responses.data_transfer).unwrap()))
                                            }
                                            Err(e) => {
                                                ctx.text(wrap_call_error_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    ErrorCode::FormatViolation,
                                                    &format!("{:#?}", e)))
                                            }
                                        }
                                    }
                                    "DiagnosticsStatusNotification" => {
                                        let response = diagnostics_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }
                                    "FirmwareStatusNotification" => {
                                        let response = firmware_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response)
                                    }
                                    "Heartbeat" => {
                                        let response = heartbeat_response(
                                            unpacked.get("MessageId").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response)
                                    }
                                    "MeterValues" => {
                                        let response = meter_values_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }
                                    "StartTransaction" => {
                                        match serde_json::from_str(&unpacked.get("Payload").unwrap()) as Result<requests::StartTransactionRequest, serde_json::Error> {
                                            Ok(_) => {
                                                let response = wrap_call_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    serde_json::to_string(&self.default_responses.start_transaction).unwrap());
                                                info!("{}: outgoing response: {}", self.name, response.clone());
                                                ctx.text(response);
                                            }
                                            Err(e) => {
                                                let response = wrap_call_error_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    ErrorCode::FormatViolation,
                                                    &format!("{:#?}", e));
                                                ctx.text(response.clone());
                                                warn!("{}: outgoing response: {}", self.name, response);

                                            }
                                        }
                                    }
                                    "StatusNotification" => {
                                        let response = status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }
                                    "StopTransaction" => {
                                        match serde_json::from_str(&unpacked.get("Payload").unwrap()) as Result<requests::StopTransactionRequest, serde_json::Error> {
                                            Ok(_) => {
                                                let response = wrap_call_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    serde_json::to_string(&self.default_responses.stop_transaction).unwrap());
                                                info!("{}: outgoing response: {}", self.name, response.clone());
                                                ctx.text(response);

                                            }
                                            Err(e) => {
                                                let response = wrap_call_error_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    ErrorCode::FormatViolation,
                                                    &format!("{:#?}", e));
                                                ctx.text(response.clone());
                                                warn!("{}: outgoing response: {}", self.name, response);
                                            }
                                        }
                                    }
                                    "LogStatusNotification" => {
                                        let response = log_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "SecurityEventNotification" => {
                                        let response = security_event_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }
                                    "SignCertificate" => {
                                        match serde_json::from_str(&unpacked.get("Payload").unwrap()) as Result<requests::SignCertificateRequest, serde_json::Error> {
                                            Ok(_) => {
                                                let response = wrap_call_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    serde_json::to_string(&self.default_responses.stop_transaction).unwrap());
                                                ctx.text(response.clone());
                                                info!("{}: outgoing response: {}", self.name, response);
                                            }
                                            Err(e) => {
                                                let response = wrap_call_error_result(
                                                    unpacked.get("MessageId").unwrap(),
                                                    ErrorCode::FormatViolation,
                                                    &format!("{:#?}", e));
                                                ctx.text(response.clone());
                                                warn!("{}: outgoing response: {}", self.name, response)
                                            }
                                        }
                                    }
                                    "SignedFirmwareStatusNotification" => {
                                        let response = signed_firmware_status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        info!("{}: outgoing response: {}", self.name, response.clone());
                                        ctx.text(response);
                                    }

                                    _ => {
                                        let response =
                                            wrap_call_error_result(
                                                unpacked.get("MessageId").unwrap(),
                                                ErrorCode::NotImplemented,
                                                &String::from(
                                                    "\"Not all messages are implemented yet. \
                                                    Ocpp server is still in development\""));
                                        ctx.text(response.clone());
                                        error!("{}", response);
                                    }
                                }
                            }
                            3 => {
                                let message_id_option = unpacked.get("MessageId");
                                let payload_option = unpacked.get("Payload");
                                if message_id_option.is_some() && payload_option.is_some() {
                                    let call_result = CallResult {
                                        unique_id: message_id_option.unwrap().clone(),
                                        payload: serde_json::from_str(payload_option.unwrap().clone().as_str()).unwrap(),
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

                                if message_id_option.is_some() && error_code_option.is_some() &&
                                    error_description_option.is_some() && error_details_option.is_some() {
                                    let call_error = CallError {
                                        unique_id: strip_quotes(&message_id_option.unwrap()).parse().unwrap(),
                                        error_code: strip_quotes(&error_code_option.unwrap()).parse().unwrap(),
                                        error_description: strip_quotes(&error_description_option.unwrap()).parse().unwrap(),
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
                    _ => {}
                }
            }
            ws::Message::Binary(_) => warn!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop()
        }
    }
}
