use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use actix::{Actor, Addr};
use actix_files::Files;
use actix_web::{App, Error as ActixWebError, get, HttpRequest, HttpResponse, HttpServer, post,
                Responder, web};
use actix_web_actors::ws;
use dotenv;
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rustls::{AllowAnyAnonymousOrAuthenticatedClient, RootCertStore};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use serde::Serialize;

mod config;
mod messages;
mod server;
mod charger_client;
mod webclient;
mod error;
mod database;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 1] = ["ocpp1.6"];

#[derive(Serialize)]
struct Status {
    status: &'static str,
}

#[get("/ocpp/{serial_id}")]
async fn ws_ocpp_index(r: HttpRequest,
                       stream: web::Payload,
                       srv: web::Data<Addr<server::OcppServer>>,
                       db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let conn = db.get().unwrap();
            database::add_charger(&conn, serial_id).expect("Could not add charger to the dtaabase");
            ws::start_with_protocols(
                charger_client::ChargeStationWebSocketSession {
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    address: srv.get_ref().clone(),
                    default_responses: charger_client::DefaultResponses {
                        authorize: messages::responses::AuthorizeResponse {
                            id_tag_info: messages::responses::IdTagInfo {
                                expiry_date: None,
                                parent_id_tag: None,
                                status: messages::responses::IdTagInfoStatus::Accepted,
                            }
                        },
                        data_transfer: messages::responses::DataTransferResponse {
                            data: None,
                            status: messages::responses::DataTransferStatus::Accepted,
                        },
                        sign_certificate: messages::responses::SignCertificateResponse {
                            status: messages::responses::GenericStatusEnumType::Accepted
                        },
                        start_transaction: messages::responses::StartTransactionResponse {
                            id_tag_info: messages::responses::IdTagInfo {
                                expiry_date: None,
                                parent_id_tag: None,
                                status: messages::responses::IdTagInfoStatus::Accepted,
                            },
                            transaction_id: 123,
                        },
                        stop_transaction: messages::responses::StopTransactionResponse {
                            id_tag_info: None
                        },
                    },
                }, &ALLOWED_SUB_PROTOCOLS, &r, stream)
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/api/webclient-socket/{serial_id}")]
async fn ws_webclient_index(r: HttpRequest,
                            stream: web::Payload,
                            srv: web::Data<Addr<server::OcppServer>>) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            ws::start(webclient::WebBrowserWebSocketSession {
                id: String::from(serial_id),
                hb: Instant::now(),
                address: srv.get_ref().clone(),
            }, &r, stream)
        }
        None => Err(ActixWebError::from(HttpResponse::BadRequest()))
    }
}

#[get("/api/get-chargers")]
async fn get_chargers(srv: web::Data<Addr<server::OcppServer>>) -> Result<impl Responder, error::Error> {
    //Ok(web::Json(vec!["charger1", "charger2", "charger3", "charger4"]).with_header("Access-Control-Allow-Origin", "*"))
    match srv.send(server::GetChargers).await {
        Ok(chargers) => Ok(web::Json(chargers).with_header("Access-Control-Allow-Origin", "*")),
        Err(e) => {
            error!("{:#?}", e);
            Err(error::Error { message: "Unable to get list of chargers".to_string(), status: 500 })
        }
    }
}

#[post("/api/post-request")]
async fn post_request(srv: web::Data<Addr<server::OcppServer>>,
                      item: web::Json<server::MessageFromWebBrowser>) -> HttpResponse {
    match srv.send(item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(Status { status: "0k" }),
        Err(_) => HttpResponse::Ok().json(Status { status: "not 0k" })
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = fs::create_dir("./logs");
    env_logger::init();
    match database::create_database(){
        Err(e) => {
            error!("Unable to create database logs.db. Reason: {:#?}", e);
        }
        _ => {}
    }

    let manager = SqliteConnectionManager::file("logs.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    if config.server.use_tls {
        info!("Server is listening.\r\n \
              Open web-browser with the url https://{host}:{port}/\r\n \
              Connect chargers with the url wss://{host}:{port}/ocpp/",
              host = config.server.host, port = config.server.port);
    } else {
        info!("Server is listening.\r\n \
               Open web-browser with the url http://{host}:{port}/\r\n \
               Connect chargers with the url ws://{host}:{port}/ocpp/",
               host = config.server.host, port = config.server.port);
    }
    let ocpp_server = server::OcppServer::new().start();
    let http_server = HttpServer::new(move || {
        App::new()
            .data(ocpp_server.clone())
            .data(pool.clone())
            //.service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/logs", "./logs/"))
            .service(get_chargers)
            .service(post_request)
            .service(ws_ocpp_index)
            .service(ws_webclient_index)
            .service(Files::new("/", "./webclient/").index_file("index.html"))
    });
    if config.server.use_tls {
        // TODO: TLS is not working at the moment
        let root_cert_store = RootCertStore::empty();
        let mut tls_config = rustls::ServerConfig::new(
            AllowAnyAnonymousOrAuthenticatedClient::new(root_cert_store));
        let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
        let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();
        tls_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
        http_server.bind_rustls(format!("{}:{}", config.server.host, config.server.port), tls_config)?
            //.bind(format!("{}:{}", config.server.host, config.server.port))?
            .run()
            .await
    } else {
        http_server.bind(format!("{}:{}", config.server.host, config.server.port))?
            .run()
            .await
    }
}
