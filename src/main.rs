#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
//use awmp::Parts;
//use std::collections::HashMap;
use dotenv;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Serialize};
use self::models::*;
use handlebars::Handlebars;

mod requests;
mod responses;
mod schema;
mod models;

use self::schema::available_chargers::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    available_chargers: Vec<self::models::AvailableCharger>
}

async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>)
               -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    let available_chargers_data = web::block(move ||
        available_chargers.load::<AvailableCharger>(&connection))
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;
    let data = IndexTemplateData {
        available_chargers: available_chargers_data
    };
    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/").unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    dotenv::from_filename("settings.env").ok();
    let env_database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let env_ip_address = dotenv::var("IP_ADDRESS").expect("IP_ADDRESS must be set");
    let env_port = dotenv::var("PORT").expect("PORT must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(&env_database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("server is listening on ip address {} and port {}", &env_ip_address, &env_port);
    HttpServer::new(move || {
        App::new().app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    }).bind(format!("{}:{}", &env_ip_address, &env_port))?.run().await
}