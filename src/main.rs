use std::time::Instant;
use actix_files::Files;
use actix::{Actor, Addr};
use actix_web::{App, Error as ActixWebError, get, HttpRequest, HttpResponse, HttpServer, web, Responder};
use actix_web_actors::ws;
use dotenv;

mod config;
mod messages;
mod server;
mod client;
mod error;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 1] = ["ocpp2.0.1"];

#[get("/ocpp/{serial_id}")]
async fn ws_ocpp_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let res = ws::start_with_protocols(
                client::ChargeStationWebSocketSession {
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    address: srv.get_ref().clone(),
                }, &ALLOWED_SUB_PROTOCOLS, &r, stream);
            res
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/webclient-socket")]
async fn ws_webclient_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, ActixWebError> {
    ws::start(client::WebBrowserWebSocketSession {hb: Instant::now() }, &r, stream)
}


#[get("/api/get-chargers")]
async fn get_chargers(srv: web::Data<Addr<server::OcppServer>>) -> Result<impl Responder, error::Error> {
    match srv.send(server::GetChargers).await {
        Ok(chargers) => Ok(web::Json(chargers).with_header("Access-Control-Allow-Origin", "*")),
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
            .service(get_chargers)
            .service(ws_ocpp_index)
            .service(ws_webclient_index)
            .service(Files::new("/", "./webclient/").index_file("index.html"))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
