use std::time::Instant;
use actix::{Actor, Addr};
use actix_files::Files;
use actix_web::{App, Error as ActixWebError, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use actix_web_actors::ws;
use dotenv;
use serde::Serialize;

mod config;
mod messages;
mod server;
mod charger_client;
mod webclient;
mod error;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 1] = ["ocpp1.6"];

#[derive(Serialize)]
struct Status{
    status: &'static str
}

#[get("/ocpp/{serial_id}")]
async fn ws_ocpp_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let res = ws::start_with_protocols(
                charger_client::ChargeStationWebSocketSession {
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    address: srv.get_ref().clone(),
                }, &ALLOWED_SUB_PROTOCOLS, &r, stream);
            res
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/api/webclient-socket/{serial_id}")]
async fn ws_webclient_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            ws::start(webclient::WebBrowserWebSocketSession {
                id: String::from(serial_id),
                hb: Instant::now(),
                address: srv.get_ref().clone()}, &r, stream)
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/api/get-chargers")]
async fn get_chargers(srv: web::Data<Addr<server::OcppServer>>) -> Result<impl Responder, error::Error> {
    //Ok(web::Json(vec!["charger1", "charger2", "charger3", "charger4"]).with_header("Access-Control-Allow-Origin", "*"))
    match srv.send(server::GetChargers).await {
        Ok(chargers) => Ok(web::Json(chargers).with_header("Access-Control-Allow-Origin", "*")),
        Err(_) => Err(error::Error{ message: "Unable to get list of chargers".to_string(), status: 500 })
    }
}

#[post("/api/post-request")]
async fn post_request(srv: web::Data<Addr<server::OcppServer>>,
                      item: web::Json<server::MessageFromWebBrowser>) -> HttpResponse {
    match srv.send(item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(Status{ status: "0k" }),
        Err(_) => HttpResponse::Ok().json(Status{ status: "not 0k" })
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
            .service(post_request)
            .service(ws_ocpp_index)
            .service(ws_webclient_index)
            .service(Files::new("/", "./webclient/").index_file("index.html"))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
