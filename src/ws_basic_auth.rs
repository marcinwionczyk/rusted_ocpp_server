use actix_web::{http, Error as ActixWebError, HttpRequest, HttpResponse};
use http_auth_basic::Credentials;
use log::{debug, error};

/// Websocket basic authentication as described in RFC 2617. I'm checking if I have AUTHORIZATION
/// header in http request matching witch chargepoint_id and authorization_key
pub fn ws_basic_auth(
    chargepoint_id: &str,
    authorization_key: &str,
    r: HttpRequest,
) -> Result<(), ActixWebError> {
    if (!authorization_key.is_empty()) && (!chargepoint_id.is_empty()) {
        if let Some(authorization_header_value) = r.headers().get(http::header::AUTHORIZATION) {
            // authorization_header_value is &HeaderValue type and we have to extract &str from it
            if let Ok(authorization_header) = authorization_header_value.to_str() {
                // if we are successful extract credentials from authorization_header
                return match Credentials::from_header(String::from(authorization_header)) {
                    Ok(credentials) => {
                        debug!(
                            "user_id: {}, password: {}",
                            credentials.user_id, credentials.password
                        );
                        // if we are successful compare credentials against function parameters
                        if credentials.user_id == String::from(chargepoint_id)
                            && credentials.password == String::from(authorization_key)
                        {
                            Ok(())
                        } else {
                            Err(ActixWebError::from(HttpResponse::Unauthorized().reason("Wrong credentials").finish()))
                        }
                    }
                    Err(e) => {
                        error!("Authentication error: {}", e);
                        Err(ActixWebError::from(HttpResponse::BadRequest().finish()))
                    }
                }
            }
        } else {
            return Err(ActixWebError::from(
                HttpResponse::BadRequest()
                    .reason("No AUTHORIZATION header in HTTP request")
                    .finish(),
            ));
        }
    }
    Ok(())
}
