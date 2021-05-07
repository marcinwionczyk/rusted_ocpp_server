use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv;
use serde::{Deserialize, Serialize};

use crate::requests::*;

mod config;
mod requests;
mod responses;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const ALLOWED_SUBPROTOCOLS: [&'static str; 3] = ["ocpp1.6", "ocpp2.0", "ocpp2.0.1"];

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
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop()
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
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8")
        .body(include_str!("../static/forms.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    println!("server is listening on ip address {} and port {}",
             config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            //.data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(ws_index)
    }).bind(format!("{}:{}", config.server.host, config.server.port))?.run().await
}