use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use crate::messages::*;
use crate::server;
use actix_web_actors::ws::{ProtocolError};
use crate::server::{MessageToWebBrowser, ConnectWebClient, DisconnectWebClient};
use serde_json::Value;

pub struct WebBrowserWebSocketSession {
    pub id: String,
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
        let addr = ctx.address();
        self.address.send(ConnectWebClient{
            addr: addr.recipient(),
            serial_id: self.id.clone(),
        }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with the ocpp server
                    _ => ctx.stop(),
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.address.do_send(server::DisconnectWebClient{ serial_id: self.id.clone() });
        Running::Stop
    }
}


impl Handler<server::MessageToWebBrowser> for WebBrowserWebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: MessageToWebBrowser, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg) {
            Ok(message_to_web_browser) => {ctx.text(message_to_web_browser)}
            Err(_) => {}
        }
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
                if message.is_some() {
                    match message.unwrap().as_str() {
                        None => {}
                        Some("connect") => {
                            match serde_json::to_string(&MessageToWebBrowser{ message: "connected to the ocpp server".to_string() }){
                                Ok(text) => ctx.text(text),
                                Err(_) => {}
                            }
                        }
                        Some("disconnect") => {
                            match serde_json::to_string(&MessageToWebBrowser{ message: "disconnecting from the ocpp server".to_string() }){
                                Ok(text) => {
                                    ctx.text(text)
                                },
                                Err(_) => {}
                            }
                            self.address.do_send(
                                DisconnectWebClient{
                                    serial_id: self.id.clone()
                                });
                        }
                        _ => {}
                    }
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Nop => (),
            _ => ctx.stop()
        }
    }
}
