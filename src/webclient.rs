use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use crate::messages::*;
use crate::server;
use actix_web_actors::ws::ProtocolError;
use crate::server::MessageToWebBrowser;
use serde_json::Value;
use uuid::Uuid;

pub struct WebBrowserWebSocketSession {
    pub id: Uuid,
    pub hb: Instant,
    pub address: Addr<server::OcppServer>,
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
                let json: Value = serde_json::from_str(text.as_str()).expect("JSON string is wrong");
                let message = json.get("message");
                let client_id = json.get("clientId");
                if message.is_some() && client_id.is_some() {
                    match message.unwrap().as_str() {
                        None => {}
                        Some("connect") => {
                            match client_id.unwrap().as_str() {
                                None => {}
                                Some(uuid) => {
                                    match Uuid::parse_str(uuid) {
                                        Ok(parsed_uuid) => {
                                            self.id = parsed_uuid;
                                            let mut text = "{\"message\":\"Connected to the Ocpp server. Your client id is ".to_string();
                                            text += parsed_uuid.to_string().as_str();
                                            text += " \"}".to_string().as_str();
                                            ctx.text(text);
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                        Some("disconnect") => {

                        }
                        _ => {}
                    }
                }
            }
            _ => ctx.stop()
        }
    }
}
