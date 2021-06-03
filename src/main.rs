use std::time::Instant;

use actix::{Actor, Addr};
use actix_web::{App, Error as ActixWebError, get, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use actix_web_actors::ws;
use dotenv;
use qstring::QString;
use crate::server::{GetChargers, WebClientMessage};

mod config;
mod messages;
mod server;
mod client;
mod error;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 1] = ["ocpp2.0.1"];


#[get("/ocpp/{serial_id}")]
async fn ws_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let res = ws::start_with_protocols(
                client::ChargePointWebSocketSession {
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    addr: srv.get_ref().clone(),
                }, &ALLOWED_SUB_PROTOCOLS, &r, stream);
            res
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/")]
async fn index(r: HttpRequest, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    let q = r.query_string();
    if q.len() > 0 {
        println!("query string: {}", r.query_string());
        let qs = QString::from(q);
        let charge_point_name = qs.get("charge_point");
        let choice_name = qs.get("choice");
        if charge_point_name.is_some() && choice_name.is_some() {
            srv.do_send(WebClientMessage {
                serial_id: charge_point_name.unwrap().to_string(),
                text: "".to_string(),
            });
        }
        Ok(HttpResponse::Ok().body(r#"<h1>Content passed to ocpp server</h1><p><a href="/">Go back</a></p>"#))
    } else {
        Ok(
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(include_str!("../static/index-old.html")))
    }
}

#[get("/api/get-chargers")]
async fn get_chargers(srv: web::Data<Addr<server::OcppServer>>) -> Result<Json<Vec<String>>, error::Error> {
    match srv.send(GetChargers).await {
        Ok(chargers) => Ok(web::Json(chargers)),
        Err(_) => Err(error::Error{ message: "Unable to get list of chargers".to_string(), status: 500 })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    println!("Server is listening.\r\n \
              Open web-browser with the url http://{host}:{port}/\r\n \
              Connect chargers with the url ws://{host}:{port}/ocpp/",
             host = config.server.host, port = config.server.port);
    let server = server::OcppServer::new().start();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            //.data(pool.clone())
            //.service(web::resource("/").route(web::get().to(index)))
            .service(index)
            .service(get_chargers)
            .service(ws_index)
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
