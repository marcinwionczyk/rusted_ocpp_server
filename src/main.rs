use std::collections::HashMap;
use std::time::Instant;

use actix::dev::MessageResponse;
use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, web};
use actix_web_actors::ws;
use dotenv;

use crate::messages::{CLIENT_TIMEOUT, HEARTBEAT_INTERVAL, unpack};
use actix::{Addr, Actor};

mod config;
mod messages;
mod server;
mod client;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 2] = ["ocpp1.6", "ocpp2.0"];

#[get("/ocpp/{serial_id}")]
async fn ws_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, Error> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let res = ws::start_with_protocols(
                client::ChargePointWebSocketSession{
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    addr: srv.get_ref().clone()
                }, &ALLOWED_SUB_PROTOCOLS, &r, stream);
            res
        }
        None => Err(Error::from(HttpResponse::BadRequest()))
    }
}


async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index-old.html")))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    println!(
        "server is listening on ip address {} and port {}",
        config.server.host, config.server.port
    );
    let server = server::OcppServer::new().start();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            //.data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(ws_index)
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
