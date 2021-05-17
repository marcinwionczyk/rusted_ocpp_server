use actix::prelude::*;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv;
use std::time::Instant;
use crate::messages::{HEARTBEAT_INTERVAL, CLIENT_TIMEOUT, unpack};

mod config;
mod messages;
mod responses;
mod requests;

const ALLOWED_SUBPROTOCOLS: [&'static str; 2] = ["ocpp1.6", "ocpp2.0"];

struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT)
    /// otherwise the connection will b e dropped
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }
    /// helper method that sends ping to client every second.
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(messages::HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                match unpack(&text) {
                    Ok(unpacked) => {
                        let message_type_id: u8 = unpacked.get("MessageTypeId").unwrap().parse().unwrap();                        
			            // println!("Identified message_type_id: {}", message_type_id);
                        match message_type_id {
                            2 => {
                                let action: &str = &unpacked.get("Action").unwrap().as_str().replace("\"", "");

                                println!("Identified Action: {}", action);
                                match action {
                                    "BootNotification" => {
                                        let response = messages::boot_notification_response(
                                            unpacked.get("MessageId").unwrap(), unpacked.get("Payload").unwrap());
                                        println!("response: {}", response);
                                        ctx.text(response)},
                                    "StatusNotification" => {
                                        let response = messages::status_notification_response(
                                            unpacked.get("MessageId").unwrap(), unpacked.get("Payload").unwrap());
                                        ctx.text(response);
                                    },
                                    "Heartbeat" => {
                                        let response = messages::heartbeat_response(unpacked.get("MessageId").unwrap());
                                        println!("response: {}", response);
                                        ctx.text(response);
                                    },
                                    "Authorize" => {
                                        let response = messages::authorize_response(unpacked.get("MessageId").unwrap(), 
                                            unpacked.get("Payload").unwrap());
                                        ctx.text(response);
                                    }
                                    _ => {}
                                }
                            },
                            3 => {

                            },
                            4 => {

                            }
                            _ => { ctx.text(text)}
                        }
                    }
                    Err(_) => {}
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[get("/ws/{charger_id}")]
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start_with_protocols(MyWebSocket::new(), &ALLOWED_SUBPROTOCOLS, &r, stream);
    println!("{:?}", res);
    res
}

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/forms.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    println!(
        "server is listening on ip address {} and port {}",
        config.server.host, config.server.port
    );
    HttpServer::new(move || {
        App::new()
            //.data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(ws_index)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
