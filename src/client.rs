use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use crate::messages::*;
use crate::server;
use actix_web_actors::ws::ProtocolError;
use crate::messages::responses::TransactionEventResponse;
use crate::server::MessageToWebBrowser;

pub struct WebBrowserWebSocketSession {
    pub hb: Instant
}



impl WebBrowserWebSocketSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}


impl Actor for WebBrowserWebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}


impl Handler<server::MessageToWebBrowser> for WebBrowserWebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: MessageToWebBrowser, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebBrowserWebSocketSession {
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
                println!("{}", text);
            }
            _ => ctx.stop()
        }
    }
}

pub struct ChargeStationWebSocketSession {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT)
    /// otherwise the connection will b e dropped
    pub hb: Instant,
    pub name: String,
    pub address: Addr<server::OcppServer>,
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

impl ChargeStationWebSocketSession {
    /// helper method that sends ping to client every second.
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
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
        ctx.text(msg.0);
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
        println!("{}: incoming message: {:?}", self.name, msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                match unpack_ocpp_message(&text) {
                    Ok(unpacked) => {
                        let message_type_id: u8 = unpacked.get("MessageTypeId").unwrap().parse()
                            .unwrap();
                        match message_type_id {
                            2 => {
                                let action: &str = &unpacked.get("Action").unwrap().as_str()
                                    .replace("\"", "");
                                match action {
                                    "BootNotification" => {
                                        let response = boot_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response)
                                    }
                                    "StatusNotification" => {
                                        let response = status_notification_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "Heartbeat" => {
                                        let response = heartbeat_response(
                                            unpacked.get("MessageId").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "Authorize" => {
                                        let response = authorize_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "NotifyEvent" => {
                                        let response = notify_event_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "NotifyReport" => {
                                        let response = notify_report_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap());
                                        println!("{}: outgoing response: {}", self.name, response);
                                        ctx.text(response);
                                    }
                                    "TransactionEvent" => {                                         
                                        let response = transaction_event_response(
                                            unpacked.get("MessageId").unwrap(),
                                            unpacked.get("Payload").unwrap(),
                                            TransactionEventResponse{
                                                charging_priority: None,
                                                custom_data: None,
                                                id_token_info: None,
                                                total_cost: None,
                                                updated_personal_message: None
                                            }
                                        );
                                        println!("{}: outgoing response: {}", self.name, response);
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
                                        ctx.text(response);
                                    }
                                }
                            }
                            3 => {}
                            4 => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop()
        }
    }
}
