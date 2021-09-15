use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc, SecondsFormat, Duration as ChronoDuration};
use serde::Deserialize;
use serde_json::{Value};
use dotenv;

pub mod requests;
pub mod responses;

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(600);

pub enum ErrorCode {
    FormatViolation,
    // Payload for Action is syntactically incorrect
    GenericError,
    // Any other error not covered by the more specific error codes in this table
    InternalError,
    // An internal error occurred and the receiver was not able to process the requested Action successfully
    MessageTypeNotSupported,
    // A message with an Message Type Number received that is not supported by this implementation.
    NotImplemented,
    // Requested Action is not known by receiver
    NotSupported,
    // Requested Action is recognized but not supported by the receiver
    OccurrenceConstraintViolation,
    // Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints
    PropertyConstraintViolation,
    // Payload is syntactically correct but at least one field contains an invalid value
    ProtocolError,
    // Payload for Action is not conform the PDU structure
    RpcFrameworkError,
    // Content of the call is not a valid RPC Request, for example: MessageId could not be read.
    SecurityError,
    // During the processing of Action a security issue occurred preventing receiver from completing the Action successfully
    TypeConstraintViolation, // Payload for Action is syntactically correct but at least one of the fields violates data type constraints
}

#[derive(Deserialize)]
pub struct Call{
    pub unique_id: String,
    pub action: String,
    pub payload: serde_json::Value
}

#[derive(Deserialize)]
pub struct CallResult{
    pub unique_id: String,
    pub payload: serde_json::Value
}
#[derive(Deserialize)]
pub struct CallError{
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: String
}

pub fn wrap_call(message_id: &String, action: &String, payload: &String) -> String {
    let m = if message_id.starts_with("\"") && message_id.ends_with("\"") {
        format!("{}", message_id)
    } else {
        format!("\"{}\"", message_id)
    };
    let a = if action.starts_with("\"") && action.ends_with("\"") {
        format!("{}", action)
    } else {
        format!("\"{}\"", action)
    };
    format!("[2, {}, {}, {}]", m, a, payload)
}

// [<MessageTypeId>, "<UniqueId>", {<Payload>}]
pub fn wrap_call_result(unique_id: &String, payload: String) -> String {
    let m = if unique_id.starts_with("\"") && unique_id.ends_with("\"") {
        format!("{}", unique_id)
    } else {
        format!("\"{}\"", unique_id)
    };
    format!("[3, {}, {}]", m, payload)
}



// [<MessageTypeId>, "<UniqueId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub fn wrap_call_error_result(msg_id: &String, error_code: ErrorCode, error_details: &String) -> String {
    match error_code {
        ErrorCode::FormatViolation => {
            format!("[4, {}, \"FormationViolation\", \"Payload for Action is syntactically \
                incorrect or not conform the PDU structure for Action\", {}]", msg_id, error_details)
        }
        ErrorCode::GenericError => {
            format!("[4, {}, \"GenericError\", \"Non specific error\", {}]",
                    msg_id, error_details)
        }
        ErrorCode::InternalError => {
            format!("[4, {}, \"InternalError\", \"An internal error occurred and the receiver \
                was not able to process the requested Action successfully\", {}]",
                    msg_id, error_details)
        }
        ErrorCode::MessageTypeNotSupported => {
            format!("[4, {}, \"MessageTypeNotSupported\", \"A message with an Message Type \
                Number received that is not supported by this implementation\", {}]",
                    msg_id, error_details)
        }
        ErrorCode::NotImplemented => {
            format!("[4, {}, \"NotImplemented\", \"Requested Action is not known by receiver\", {}]",
                    msg_id, error_details)
        }
        ErrorCode::NotSupported => {
            format!("[4, {}, \"NotSupported\", \"Requested Action is recognized but not supported \
                by the receiver\", {}]", msg_id, error_details)
        }
        ErrorCode::OccurrenceConstraintViolation => {
            format!("[4, {}, \"OccurrenceConstraintViolation\", \"Payload for Action is \
                syntactically correct but at least one of the fields violates occurrence \
                constraints\", {}]", msg_id, error_details)
        }
        ErrorCode::PropertyConstraintViolation => {
            format!("[4, {}, \"PropertyConstraintViolation\", \"Payload for Action is \
                syntactically correct but at least one of the fields violates occurrence \
                constraints\", {}]", msg_id, error_details)
        }
        ErrorCode::ProtocolError => {
            format!("[4, {}, \"ProtocolError\", \"Payload for Action is not conform the PDU \
                structure\", {}]", msg_id, error_details)
        }
        ErrorCode::RpcFrameworkError => {
            format!("[4, {}, \"RpcFrameworkError\", \"Content of the call is not a valid RPC Request, \
            for example: MessageId could not be read.\", {}]", msg_id, error_details)
        }
        ErrorCode::SecurityError => {
            format!("[4, {}, \"SecurityError\", \"During the processing of Action a security issue \
            occurred preventing receiver from completing the Action successfully\", {}]",
                    msg_id, error_details)
        }
        ErrorCode::TypeConstraintViolation => {
            format!("[4, {}, \"TypeConstraintViolation\", \"Payload for Action is syntactically \
            correct but at least one of the fields violates data type constraints\", {}]",
                    msg_id, error_details)
        }
    }
}

pub fn unpack_ocpp_message(msg: &String) -> Result<HashMap<&str, String>, String> {
    let mut hash: HashMap<&str, String> = HashMap::new();
    let json: Value = serde_json::from_str(msg).expect("JSON string is wrong");
    let message_type_id = json.get(0).unwrap().as_u64();
    match message_type_id {
        Some(2) => {
            hash.insert("MessageTypeId", "2".to_string());
            hash.insert("MessageId", (json.get(1).unwrap()).to_string());
            hash.insert("Action", (json.get(2).unwrap()).to_string());
            hash.insert("Payload", (json.get(3).unwrap()).to_string());
            Ok(hash.clone())
        }
        Some(3) => {
            hash.insert("MessageTypeId", "3".to_string());
            hash.insert("MessageId", (json.get(1).unwrap()).to_string());
            hash.insert("Payload", (json.get(2).unwrap()).to_string());
            Ok(hash.clone())
        }
        Some(4) => {
            hash.insert("MessageTypeId", "4".to_string());
            hash.insert("MessageId", (json.get(1).unwrap()).to_string());
            hash.insert("ErrorCode", (json.get(2).unwrap()).to_string());
            hash.insert("ErrorDescription", (json.get(3).unwrap()).to_string());
            hash.insert("ErrorDetails", (json.get(4).unwrap()).to_string());
            Ok(hash.clone())
        }
        None => {
            let message_id = (json.get(1).unwrap()).to_string();
            Err(wrap_call_error_result(&message_id, ErrorCode::MessageTypeNotSupported, msg))
        }
        _ => {
            let message_id = (json.get(1).unwrap()).to_string();
            Err(wrap_call_error_result(&message_id, ErrorCode::MessageTypeNotSupported, msg))
        }
    }
}

pub fn boot_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::BootNotificationRequest, serde_json::Error> {
        Ok(_) => {
            dotenv::from_filename("settings.env").ok();
            let config = crate::config::Config::from_env().unwrap();
            let at_now:DateTime<Utc> = Utc::now() + ChronoDuration::hours(config.server.time_offset);
            let boot_response: responses::BootNotificationResponse = responses::BootNotificationResponse {
                current_time: at_now.to_rfc3339_opts(SecondsFormat::Millis, false),
                interval: config.server.heartbeat_interval,
                status: responses::BootNotificationStatus::Accepted,
            };

            wrap_call_result(message_id, serde_json::to_string(&boot_response).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn start_transaction_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::StartTransactionRequest, serde_json::Error> {
        Ok(_) => {
            let start_transaction_response = responses::StartTransactionResponse{
                id_tag_info: responses::IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: responses::IdTagInfoStatus::Accepted
                },
                transaction_id: 1500100900
            };
            wrap_call_result(message_id, serde_json::to_string(&start_transaction_response).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn stop_transaction_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::StopTransactionRequest, serde_json::Error> {
        Ok(_) => {
            let stop_transaction_response = responses::StopTransactionResponse{
                id_tag_info: None
            };
            wrap_call_result(message_id, serde_json::to_string(&stop_transaction_response).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}



pub fn authorize_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::AuthorizeRequest, serde_json::Error> {
        Ok(_) => {
            let authorize_resp =  responses::AuthorizeResponse{
                id_tag_info: responses::IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: responses::IdTagInfoStatus::Accepted
                }
            };
            wrap_call_result(message_id, serde_json::to_string(&authorize_resp).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn heartbeat_response(message_id: &String) -> String {
    let config = crate::config::Config::from_env().unwrap();
    let at_now:DateTime<Utc> = Utc::now() + ChronoDuration::hours(config.server.time_offset);
    let heartbeat_resp: responses::HeartbeatResponse = responses::HeartbeatResponse {
        current_time: at_now.to_rfc3339_opts(SecondsFormat::Millis, false),
    };
    wrap_call_result(message_id, serde_json::to_string(&heartbeat_resp).unwrap())
}

pub fn diagnostics_status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::DiagnosticsStatusNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let diagnostics_status_notification_resp = responses::DiagnosticsStatusNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&diagnostics_status_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn firmware_status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::FirmwareStatusNotificationRequest,
            serde_json::Error> {
        Ok(_) => {
            let firmware_status_notification_resp = responses::FirmwareStatusNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&firmware_status_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn meter_values_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::MeterValuesRequest, serde_json::Error> {
        Ok(_) => {
            let meter_values_resp = responses::MeterValuesResponse{};
            wrap_call_result(message_id, serde_json::to_string(&meter_values_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::StatusNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let status_notification_resp = responses::StatusNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&status_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn log_status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::LogStatusNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let log_status_notification_resp = responses::LogStatusNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&log_status_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn security_event_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::SecurityEventNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let security_event_notification_resp = responses::SecurityEventNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&security_event_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn signed_firmware_status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::SignedFirmwareStatusNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let signed_firmware_status_notification_resp = responses::SignedFirmwareStatusNotificationResponse{};
            wrap_call_result(message_id, serde_json::to_string(&signed_firmware_status_notification_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}
