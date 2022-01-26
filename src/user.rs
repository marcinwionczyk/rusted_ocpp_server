use actix_identity::Identity;
use actix_web::{ web, HttpResponse };
use serde::{Deserialize, Serialize};

use log::{info, debug};


#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    login_id: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    allowed: bool,
}

/*
curl -i --request POST \
  --url http://localhost:5000/api/auth \
  --header 'content-type: application/json' \
  --data '{"login_id": "sarah"}'
 */
pub async fn login(id: Identity, entry: web::Json<Entry>) -> web::Json<User> {
    let config = crate::config::Config::from_yaml("./settings.yaml").unwrap();
    let login_id = entry.login_id.clone().replace("\"", "");
    debug!("config.users: {:#?}", config.users);
    if config.users.contains(&login_id) {
        info!("username {} was accepted and logged in", &login_id);
        id.remember(login_id.to_owned());
        web::Json(User {
            id: login_id,
            allowed: true
        })
    } else {
        info!("username {} was not accepted", &login_id);
        web::Json(User {
            id: login_id,
            allowed: false
        })
    }
}

/*
curl -i --request DELETE \
  --url http://localhost:5000/api/auth \
  --header 'content-type: application/json'
 */
pub async fn logout(id: Identity) -> HttpResponse {
    info!("[user] ++++ logout()");
    id.forget();
    HttpResponse::Ok().finish()
}