use actix_web::{http, Error as ActixWebError, HttpRequest, HttpResponse};
use log::debug;


pub fn basic_auth(
    chargepoint_id: &str,
    authorization_key: &str,
    r: HttpRequest,
) -> Result<(), ActixWebError> {
    // Basic Auth RFC 2617. First I'm checking if I have AUTHORIZATION header in http request
    if (!authorization_key.is_empty()) && (!chargepoint_id.is_empty()) {
        if let Some(authorization_header_value) = r.headers().get(http::header::AUTHORIZATION) {
            if let Ok(authorization_header) = authorization_header_value.to_str() {
                let authorization_header_string = String::from(authorization_header);
                if let Some(index) = &authorization_header_string.find("Basic") {
                    let base64credentials =
                        authorization_header_string[(index + "Basic".len())..].trim();
                    if let Ok(authorization_decoded) = base64::decode(base64credentials) {
                        if let Ok(authorization_decoded_as_string) =
                            String::from_utf8(authorization_decoded.clone())
                        {
                            let vec_authorization: Vec<&str> =
                                authorization_decoded_as_string.split(':').collect();
                            return if vec_authorization.len() == 2 {
                                debug!("ChargePointId: {}", vec_authorization[0]);
                                debug!("AuthorizationKey: {}", vec_authorization[1]);
                                if !vec_authorization[1].eq(authorization_key)
                                    || !vec_authorization[0].eq(chargepoint_id)
                                {
                                    debug!("You are not allowed to enter with this ChargePointId and/or AuthorizationKey. It does not match.");
                                    Err(ActixWebError::from(HttpResponse::Unauthorized().reason("AUTHORIZATION header: You are not allowed to enter with this ChargePointId and/or AuthorizationKey").finish()))
                                } else {
                                    Ok(())
                                }
                            } else {
                                debug!(
                                    "AUTHORIZATION header decoded {} does not contain \":\" sign",
                                    String::from_utf8(authorization_decoded).unwrap()
                                );
                                Err(ActixWebError::from(HttpResponse::BadRequest().reason("AUTHORIZATION header when decoded from base64 does not contain \":\" sign").finish()))
                            };
                        }
                    }
                } else {
                    debug!("AUTHORIZATION header value does not contain \"Basic\" substring");
                    return Err(ActixWebError::from(
                        HttpResponse::BadRequest()
                            .reason(
                                "AUTHORIZATION header value does not contain \"Basic\" substring",
                            )
                            .finish(),
                    ));
                }
            } else {
                debug!("AUTHORIZATION header value does not contain \"Basic\" substring");
                return Err(ActixWebError::from(
                    HttpResponse::BadRequest()
                        .reason("AUTHORIZATION header value not in a form of string")
                        .finish(),
                ));
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
