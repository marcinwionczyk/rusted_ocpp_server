use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_files::Files;
use actix_http::http::header;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::cookie::SameSite;
use actix_web::{
    web, App, Error as ActixWebError, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use dotenv;
use log::{debug, error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::NoClientAuth;
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::time::Instant;

mod charger_client;
mod config;
mod error;
mod json_rpc;
mod logs;
mod messages;
mod server;
mod user;
mod webclient;
mod ws_basic_auth;

const ALLOWED_SUB_PROTOCOLS: [&'static str; 1] = ["ocpp1.6"];

async fn ws_ocpp(
    r: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::OcppServer>>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => {
            let conn = db.get().unwrap();
            logs::add_charger(&conn, serial_id).expect("Could not add charger to the database");
            let config = config::Config::from_env().unwrap();
            let ocpp_pass_auth = config.server.ocpp_auth_password;
            {
                if !ocpp_pass_auth.is_empty() {
                    if let Err(e) =
                        ws_basic_auth::ws_basic_auth(&serial_id, ocpp_pass_auth.as_str(), r.clone())
                    {
                        return Err(e);
                    }
                }
            }
            ws::start_with_protocols(
                charger_client::ChargeStationWebSocketSession {
                    hb: Instant::now(),
                    name: String::from(serial_id),
                    address: srv.get_ref().clone(),
                    db_connection: conn,
                    default_responses: charger_client::DefaultResponses::default(),
                },
                &ALLOWED_SUB_PROTOCOLS,
                &r,
                stream,
            )
        }
        None => Err(ActixWebError::from(
            HttpResponse::BadRequest()
                .reason("Charger serial id was not provided")
                .finish(),
        )),
    }
}

async fn ws_webclient(
    r: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::OcppServer>>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, ActixWebError> {
    match r.match_info().get("serial_id") {
        Some(serial_id) => ws::start(
            webclient::WebBrowserWebSocketSession {
                id: String::from(serial_id),
                hb: Instant::now(),
                address: srv.get_ref().clone(),
                db_connection: db.get().unwrap(),
            },
            &r,
            stream,
        ),
        None => Err(ActixWebError::from(HttpResponse::BadRequest())),
    }
}

async fn get_chargers(
    srv: web::Data<Addr<server::OcppServer>>,
) -> Result<impl Responder, error::Error> {
    if let Some(user_id) = id.identity() {
        // TODO: If I pass identity I might limit access to this API call
        debug!("get chargers. user id: {}", user_id)
    }
    match srv.send(server::GetChargers).await {
        Ok(chargers) => Ok(web::Json(chargers).with_header("Access-Control-Allow-Origin", "*")),
        Err(e) => {
            error!("{:#?}", e);
            Err(error::Error {
                message: "Unable to get list of chargers".to_string(),
                status: 500,
            })
        }
    }
}

async fn post_request(
    id: Identity,
    srv: web::Data<Addr<server::OcppServer>>,
    item: web::Json<server::MessageFromWebBrowser>,
) -> HttpResponse {
    if let Some(user_id) = id.identity() {
        // TODO: If I pass identity I might limit access to this API call
        debug!("post request. User id: {}", user_id);
    }
    match srv.send(item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let _ = create_dir("./logs");
    env_logger::init();
    if let Err(e) = logs::create_database() {
        error!("Unable to create database logs.db. Reason: {:#?}", e);
    }
    let manager = SqliteConnectionManager::file("logs.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    dotenv::from_filename("settings.env").ok();
    let config = crate::config::Config::from_env().unwrap();
    let domain_name = format!("{}:{}", config.server.host, config.server.port);
    let http_origin = format!(
        "{}://{}",
        if config.server.use_tls {
            "https"
        } else {
            "http"
        },
        domain_name.clone()
    );
    let ws_origin = format!(
        "{}://{}/ocpp/",
        if config.server.use_tls { "wss" } else { "ws" },
        domain_name.clone()
    );
    info!(
        "Server is listening.\r\n \
              Open web-browser with the url {}/\r\n \
              Connect chargers with the url {}",
        http_origin.clone(),
        ws_origin.clone()
    );
    let ocpp_server = server::OcppServer::new().start();
    let http_server = HttpServer::new(move || {
        App::new()
            .data(ocpp_server.clone())
            .data(pool.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(http_origin.as_str())
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .supports_credentials(), // Allow the cookie auth.
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .domain(domain_name.clone())
                    .name("krakuski")
                    .path("/")
                    .same_site(SameSite::Strict)
                    .secure(true),
            ))
            //.service(web::resource("/").route(web::get().to(index)))
            .service(
                web::scope("/api")
                    .service(
                    web::resource("/auth")
                        .route(web::post().to(user::login))
                        .route(web::delete().to(user::logout))
                )
                    .service(web::resource("/get-chargers").route(web::get().to(get_chargers)))
                    .service(web::resource("/post-request").route(web::post().to(post_request)))
                    .service(web::resource("/webclient-socket/{serial_id}").route(web::get().to(ws_webclient)))
                    .route("/", web::get().to(|| HttpResponse::Ok().body("api")))
            )
            .service(web::resource("/ocpp/{serial_id}").route(web::get().to(ws_ocpp)))
            .service(Files::new("/logs", "./logs/"))
            .service(Files::new("/", "./webclient/").index_file("index.html"))
    });
    if config.server.use_tls {
        let mut tls_config = rustls::ServerConfig::new(NoClientAuth::new());
        tls_config.alpn_protocols = vec![b"http/1.1".to_vec()];
        let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
        let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();
        if keys.is_empty() {
            error!("Could not locate PKCS 8 private keys.");
            std::process::exit(1);
        }
        if let Err(e) = tls_config.set_single_cert(cert_chain, keys.remove(0)) {
            error!("Cannot set single Cert. Reason: {:#?}", e)
        };
        http_server
            .bind_rustls(
                format!("{}:{}", config.server.host, config.server.port),
                tls_config,
            )?
            //.bind(format!("{}:{}", config.server.host, config.server.port))?
            .run()
            .await
    } else {
        http_server
            .bind(format!("{}:{}", config.server.host, config.server.port))?
            .run()
            .await
    }
}
