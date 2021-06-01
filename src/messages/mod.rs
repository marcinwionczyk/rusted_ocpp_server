use std::collections::HashMap;
use std::time::{Duration};

use chrono::{DateTime, Utc};
use serde_json::Value;

pub mod requests;
pub mod responses;

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

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

// [<MessageTypeId>, "<UniqueId>", {<Payload>}]
pub fn wrap_call_result(msg_id: &String, payload: String) -> String {
    format!("[3, {}, {}]", msg_id, payload)
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

pub fn unpack(msg: &String) -> Result<HashMap<&str, String>, String> {
    let mut hash: HashMap<&str, String> = HashMap::new();
    let json: Value = serde_json::from_str(msg).expect("JSON string is wrong");
    let message_type_id = json.get(0).unwrap().as_u64();
    match message_type_id {
        Some(2) => {
            println!("{:?}", json);
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
            let at_now:DateTime<Utc> = Utc::now();
            let boot_response: responses::BootNotificationResponse = responses::BootNotificationResponse {
                current_time: at_now.to_rfc3339(),
                custom_data: None,
                interval: HEARTBEAT_INTERVAL.as_secs() as i64,
                status: responses::RegistrationStatusEnumType::Accepted,
                status_info: None
            };
            wrap_call_result(message_id, serde_json::to_string(&boot_response).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn status_notification_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::StatusNotificationRequest, serde_json::Error> {
        Ok(_) => {
            let response = responses::StatusNotificationResponse{ custom_data: None };
            wrap_call_result(message_id, serde_json::to_string(&response).unwrap())
        }
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn heartbeat_response(message_id: &String) -> String {
    let at_now:DateTime<Utc> = Utc::now();
    let heartbeat_resp: responses::HeartbeatResponse = responses::HeartbeatResponse {
        current_time: at_now.to_rfc3339(),
        custom_data: None
    };
    wrap_call_result(message_id, serde_json::to_string(&heartbeat_resp).unwrap())
}

pub fn authorize_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::AuthorizeRequest, serde_json::Error> {
        Ok(_) => {
            let authorize_resp: responses::AuthorizeResponse = responses::AuthorizeResponse{
                certificate_status: None,
                custom_data: None,
                id_token_info: responses::IdTokenInfoType{
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    custom_data: None,
                    evse_id: None,
                    group_id_token: None,
                    language1: None,
                    language2: None,
                    personal_message: None,
                    status: responses::AuthorizationStatusEnumType::Accepted
                }
            };
            wrap_call_result(message_id, serde_json::to_string(&authorize_resp).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation, &format!("{:#?}", e))
        }
    }
}

pub fn notify_event_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::NotifyEventRequest, serde_json::Error> {
        Ok(_) => {
            let notify_event_response = responses::NotifyEventResponse{
                custom_data: None
            };
            wrap_call_result(message_id, serde_json::to_string(&notify_event_response).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation,
                                   &format!("{:#?}", e))
        }
    }
}

pub fn notify_report_response(message_id: &String, payload: &String) -> String {
    match serde_json::from_str(&payload) as Result<requests::NotifyReportRequest, serde_json::Error> {
        Ok(_) => {
            let notify_report_response = responses::NotifyReportResponse{ custom_data: None };
            wrap_call_result(message_id,
                             serde_json::to_string(&notify_report_response).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation,
                                   &format!("{:#?}", e))
        }
    }
}

pub fn transaction_event_response(message_id: &String, payload: &String,
                                  response: responses::TransactionEventResponse) -> String {
    match serde_json::from_str(&payload) as Result<requests::TransactionEventRequest, serde_json::Error> {
        Ok(_) => {
            wrap_call_result(message_id,
                             serde_json::to_string(&response).unwrap())
        },
        Err(e) => {
            wrap_call_error_result(message_id, ErrorCode::FormatViolation,
                                   &format!("{:#?}", e))
        }
    }
}