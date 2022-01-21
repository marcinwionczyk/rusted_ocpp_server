use crate::messages;
use crate::messages::{wrap_call, wrap_call_result, Call, CallError, CallResult};
use actix::prelude::*;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

// OcppServer in this file is handling multiple websocket connections, is managing them and passes
// messages
// like the postal office and a middleman between webclient and charger
// client. It is initiated in the main.rs file
//                ,_____________
//                | webclient   |
//                `____   ______'
//                     \ /
//                      |
//                   websocket
//                   worker
//                      |
//                ,_____^_______
//                |    ocpp     |
//       ,--------|   server    |-------.
//      |         `````\|````````        |
//   websocket       websocket       websocket
//   worker          worker          worker
//      |               |                |
// .----^--------. .----^---------. .----^----------.
//|charger_client| |charger_client| |charger_client |
//`--------------' `--------------' `---------------'

/// Ocpp server sends this message through websocket session to the charger
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageToChargeStation {
    pub message: Option<String>,
    pub authorize: Option<messages::responses::AuthorizeResponse>,
    pub data_transfer: Option<messages::responses::DataTransferResponse>,
    pub sign_certificate: Option<messages::responses::SignCertificateResponse>,
    pub start_transaction: Option<messages::responses::StartTransactionResponse>,
    pub stop_transaction: Option<messages::responses::StopTransactionResponse>,
}

impl Default for MessageToChargeStation {
    fn default() -> MessageToChargeStation {
        MessageToChargeStation {
            message: None,
            authorize: None,
            data_transfer: None,
            sign_certificate: None,
            start_transaction: None,
            stop_transaction: None,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromChargeStation {
    pub charger_id: String,
    pub call: Option<Call>,
    pub call_result: Option<CallResult>,
    pub call_error: Option<CallError>,
}

/// Ocpp server sends this messages through websocket session to the web browser
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct MessageToWebBrowser {
    pub message: String,
    pub payload: Option<Value>,
}

/// a OCPP message to OCPP server from web client
#[derive(Message, Clone, Deserialize)]
#[rtype(result = "()")]
pub struct MessageFromWebBrowser {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub charger: String,  // target Charge point
    pub selected: String, // action
    #[serde(rename = "messageId")]
    pub message_id: u8, // call or call_result
    pub payload: Value,   // OCPP message
}

/// New Chargepoint websocket session is created
#[derive(Message)]
#[rtype(String)]
pub struct ConnectCharger {
    pub addr: Recipient<MessageToChargeStation>,
    pub serial_id: String,
}

/// Chargepoint websocket session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectCharger {
    pub serial_id: String,
}

#[derive(Serialize)]
pub struct GetChargers;

/// New web client websocket session is created
#[derive(Message)]
#[rtype(String)]
pub struct ConnectWebClient {
    pub addr: Recipient<MessageToWebBrowser>,
    pub serial_id: String,
}

/// New web client websocket session is created
#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectWebClient {
    pub serial_id: String,
}

impl actix::Message for GetChargers {
    type Result = Vec<String>;
}

/// `OcppServer` manages websocket sessions with charge stations
pub struct OcppServer {
    awaiting_call_result: HashMap<String, String>, // key: MessageId, value: websocket_worker_id
    websocket_workers: HashMap<String, Recipient<MessageToChargeStation>>,
    webclient_workers: HashMap<String, Recipient<MessageToWebBrowser>>,
    chargers_webclients_pair: HashMap<String, String>, // key: charger_id, value: browser_id
}

impl OcppServer {
    pub fn new() -> OcppServer {
        OcppServer {
            awaiting_call_result: HashMap::new(),
            websocket_workers: HashMap::new(),
            webclient_workers: HashMap::new(),
            chargers_webclients_pair: HashMap::new(),
        }
    }

    fn send_message_to_charger(&self, charger: &String, message: MessageToChargeStation) {
        if let Some(session) = self.websocket_workers.get(charger) {
            if let Err(e) = session.do_send(message) {
                error!("{}", e.to_string());
            }
        }
    }

    fn send_message_to_web_client(
        &self,
        web_client: &String,
        message: &String,
        payload: Option<Value>,
    ) {
        if let Some(session) = self.webclient_workers.get(web_client) {
            if let Err(e) = session.do_send(MessageToWebBrowser {
                message: message.to_owned(),
                payload,
            }) {
                error!("{}", e.to_string());
            }
        }
    }

    fn _create_log(
        &self,
        charger: &String,
        time_date_start: &String,
        time_date_stop: Option<&String>,
    ) -> Result<String, std::io::Error> {
        let path = match time_date_stop {
            None => format!("./logs/{}_{}.log", charger, time_date_start),
            Some(tds) => format!("./logs/{}_{}_{}.log", charger, time_date_start, tds),
        };
        let mut _f = std::fs::File::create(path.clone())?;
        // TODO: get logs from database for specific charger and time period and write to file
        // under path
        Ok(path)
    }

    fn message_from_web_browser_is_valid(msg: MessageFromWebBrowser) -> bool {
        match msg.selected.as_str() {
            "Authorize" => {
                let res: Result<messages::responses::AuthorizeResponse, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "CancelReservation" => {
                let res: Result<messages::requests::CancelReservationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ChangeAvailability" => {
                let res: Result<messages::requests::ChangeAvailabilityRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ChangeConfiguration" => {
                let res: Result<messages::requests::ChangeConfigurationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ClearCache" => {
                let res: Result<messages::requests::ClearCacheRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ClearChargingProfile" => {
                let res: Result<
                    messages::requests::ClearChargingProfileRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "DataTransfer" => {
                let res1: Result<messages::requests::DataTransferRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload.clone());
                let res2: Result<messages::responses::DataTransferResponse, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res1.is_ok() || res2.is_ok()
            }
            "GetCompositeSchedule" => {
                let res: Result<
                    messages::requests::GetCompositeScheduleRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "GetConfiguration" => {
                let res: Result<messages::requests::GetConfigurationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "GetDiagnostics" => {
                let res: Result<messages::requests::GetDiagnosticsRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "GetLocalListVersion" => {
                let res: Result<messages::requests::GetLocalListVersionRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "RemoteStartTransaction" => {
                let res: Result<
                    messages::requests::RemoteStartTransactionRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "RemoteStopTransaction" => {
                let res: Result<
                    messages::requests::RemoteStopTransactionRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ReserveNow" => {
                let res: Result<messages::requests::ReserveNowRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "Reset" => {
                let res: Result<messages::requests::ResetRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "SendLocalList" => {
                let res: Result<messages::requests::SendLocalListRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "SetChargingProfile" => {
                let res: Result<messages::requests::SetChargingProfileRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "TriggerMessage" => {
                let res: Result<messages::requests::TriggerMessageRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "UnlockConnector" => {
                let res: Result<messages::requests::UnlockConnectorRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "UpdateFirmware" => {
                let res: Result<messages::requests::UpdateFirmwareRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "CertificateSigned" => {
                let res: Result<messages::requests::CertificateSignedRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "DeleteCertificate" => {
                let res: Result<messages::requests::DeleteCertificateRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "ExtendedTriggerMessage" => {
                let res: Result<
                    messages::requests::ExtendedTriggerMessageRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "GetInstalledCertificateIds" => {
                let res: Result<
                    messages::requests::GetInstalledCertificateIdsRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "GetLog" => {
                let res: Result<messages::requests::GetLogRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "InstallCertificate" => {
                let res: Result<messages::requests::InstallCertificateRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "SignCertificate" => {
                let res: Result<messages::responses::SignCertificateResponse, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "SignedUpdateFirmware" => {
                let res: Result<
                    messages::requests::SignedUpdateFirmwareRequest,
                    serde_json::Error,
                > = serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "StartTransaction" => {
                let res: Result<messages::responses::StartTransactionResponse, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            "StopTransaction" => {
                let res: Result<messages::responses::StopTransactionResponse, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            &_ => false,
        }
    }
}

impl Actor for OcppServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<ConnectCharger> for OcppServer {
    type Result = String;

    fn handle(&mut self, msg: ConnectCharger, _: &mut Context<Self>) -> Self::Result {
        self.websocket_workers
            .insert(msg.serial_id.clone(), msg.addr);
        info!("Inserting charger: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<ConnectWebClient> for OcppServer {
    type Result = String;

    fn handle(&mut self, msg: ConnectWebClient, _: &mut Context<Self>) -> Self::Result {
        self.webclient_workers
            .insert(msg.serial_id.clone(), msg.addr);
        info!("Inserting web client: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<DisconnectCharger> for OcppServer {
    type Result = ();
    fn handle(&mut self, msg: DisconnectCharger, _: &mut Context<Self>) -> Self::Result {
        info!("Removing charger: {}", msg.serial_id);
        self.websocket_workers.remove(msg.serial_id.as_str());
        if self
            .chargers_webclients_pair
            .contains_key(msg.serial_id.as_str())
        {
            info!("Removing charger<->webclient pair: {}", msg.serial_id);
            self.chargers_webclients_pair.remove(msg.serial_id.as_str());
        }
    }
}

impl Handler<DisconnectWebClient> for OcppServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectWebClient, _: &mut Context<Self>) -> Self::Result {
        for (item, value) in self.chargers_webclients_pair.clone() {
            if value == msg.serial_id {
                info!("Removing pair {}<->{}", &item, &value);
                self.chargers_webclients_pair.remove(&item);
            }
        }
        info!("Removing web client: {}", msg.serial_id);
        self.webclient_workers.remove(&msg.serial_id);
    }
}

impl Handler<GetChargers> for OcppServer {
    type Result = MessageResult<GetChargers>;

    fn handle(&mut self, _: GetChargers, _: &mut Context<Self>) -> Self::Result {
        let mut chargers = Vec::new();
        for key in self.websocket_workers.keys() {
            chargers.push(key.to_owned())
        }
        MessageResult(chargers)
    }
}

impl Handler<MessageFromWebBrowser> for OcppServer {
    type Result = ();

    fn handle(&mut self, msg: MessageFromWebBrowser, _: &mut Context<Self>) -> Self::Result {
        info!("sending message to: {}", msg.charger);
        if OcppServer::message_from_web_browser_is_valid(msg.clone()) {
            if msg.message_id == 2 {
                let message_id = Uuid::new_v4().to_string();
                let call = wrap_call(
                    &message_id,
                    &msg.selected,
                    &serde_json::to_string(&msg.payload).unwrap(),
                );
                let mut message_to_charge_station = MessageToChargeStation::default();
                message_to_charge_station.message = Some(call.clone());
                self.send_message_to_charger(&msg.charger, message_to_charge_station);
                self.awaiting_call_result
                    .insert(message_id, msg.client_id.clone());
                if !self
                    .chargers_webclients_pair
                    .contains_key(msg.charger.clone().as_str())
                {
                    info!(
                        "Inserting charger_webclient_pair: {} -> {}",
                        msg.charger.clone(),
                        msg.client_id.clone()
                    );
                    self.chargers_webclients_pair
                        .insert(msg.charger.clone(), msg.client_id.clone());
                }
                self.send_message_to_web_client(
                    &msg.client_id,
                    &format!("call sent to charger {}:\r\n{}", &msg.charger, call),
                    None,
                )
            }
            if msg.message_id == 3 {
                let mut message_to_charge_station = MessageToChargeStation::default();
                match msg.selected.as_str() {
                    "Authorize" => {
                        match serde_json::from_value(msg.payload.clone())
                            as Result<messages::responses::AuthorizeResponse, serde_json::Error>
                        {
                            Ok(response) => {
                                message_to_charge_station.authorize = Some(response);
                                self.send_message_to_charger(
                                    &msg.charger,
                                    message_to_charge_station,
                                );
                                info!(
                                    "Setting default Authorize response for charger {}: {}",
                                    &msg.charger,
                                    serde_json::to_string(&msg.payload).unwrap()
                                );
                                self.send_message_to_web_client(
                                    &msg.client_id,
                                    &format!(
                                        "Setting default Authorize response for charger {}:\r\n {}",
                                        &msg.charger,
                                        serde_json::to_string(&msg.payload).unwrap()
                                    ),
                                    None,
                                )
                            }
                            Err(err) => {
                                error!("Unable to parse Authorize response. Error: {:#?}", err);
                            }
                        }
                    }
                    "DataTransfer" => {
                        match serde_json::from_value(msg.payload.clone())
                            as Result<messages::responses::DataTransferResponse, serde_json::Error>
                        {
                            Ok(response) => {
                                message_to_charge_station.data_transfer = Some(response);
                                self.send_message_to_charger(
                                    &msg.charger,
                                    message_to_charge_station,
                                );
                                info!(
                                    "Setting default DataTransfer response to charger {}: {}",
                                    &msg.charger,
                                    serde_json::to_string(&msg.payload).unwrap()
                                );
                                self.send_message_to_web_client(&msg.client_id,
                                                                &format!("Setting default DataTransfer response for charger {}:\r\n {}", &msg.charger,
                                                                         serde_json::to_string(&msg.payload).unwrap()), None)
                            }
                            Err(err) => {
                                error!("Unable to parse DataTransfer response. Error: {:#?}", err);
                            }
                        }
                    }
                    "SignCertificate" => {
                        match serde_json::from_value(msg.payload.clone())
                            as Result<
                                messages::responses::SignCertificateResponse,
                                serde_json::Error,
                            > {
                            Ok(response) => {
                                message_to_charge_station.sign_certificate = Some(response);
                                self.send_message_to_charger(
                                    &msg.charger,
                                    message_to_charge_station,
                                );
                                info!(
                                    "Setting default SignCertificate response to charger {}: {}",
                                    &msg.charger,
                                    serde_json::to_string(&msg.payload).unwrap()
                                );
                                self.send_message_to_web_client(&msg.client_id,
                                                                &format!("Setting default SignCertificate response for charger {}:\r\n {}", &msg.charger,
                                                                         serde_json::to_string(&msg.payload).unwrap()), None)
                            }
                            Err(err) => {
                                error!(
                                    "Unable to parse SignCertificate response. Error: {:#?}",
                                    err
                                );
                            }
                        }
                    }
                    "StartTransaction" => {
                        match serde_json::from_value(msg.payload.clone())
                            as Result<
                                messages::responses::StartTransactionResponse,
                                serde_json::Error,
                            > {
                            Ok(response) => {
                                message_to_charge_station.start_transaction = Some(response);
                                self.send_message_to_charger(
                                    &msg.charger,
                                    message_to_charge_station,
                                );
                                info!(
                                    "Setting default StartTransaction response to charger {}: {}",
                                    &msg.charger,
                                    serde_json::to_string(&msg.payload).unwrap()
                                );
                                self.send_message_to_web_client(&msg.client_id,
                                                                &format!("Setting default StartTransaction response for charger {}:\r\n {}", &msg.charger,
                                                                         serde_json::to_string(&msg.payload).unwrap()), None)
                            }
                            Err(err) => {
                                error!(
                                    "Unable to parse StartTransaction response. Error: {:#?}",
                                    err
                                );
                            }
                        }
                    }
                    "StopTransaction" => {
                        match serde_json::from_value(msg.payload.clone())
                            as Result<
                                messages::responses::StopTransactionResponse,
                                serde_json::Error,
                            > {
                            Ok(response) => {
                                message_to_charge_station.stop_transaction = Some(response);
                                self.send_message_to_charger(
                                    &msg.charger,
                                    message_to_charge_station,
                                );
                                info!(
                                    "Setting default StopTransaction response to charger {}: {}",
                                    &msg.charger,
                                    serde_json::to_string(&msg.payload).unwrap()
                                );
                                self.send_message_to_web_client(&msg.client_id,
                                                                &format!("Setting default StopTransaction response for charger {}:\r\n {}", &msg.charger,
                                                                         serde_json::to_string(&msg.payload).unwrap()), None)
                            }
                            Err(err) => {
                                error!(
                                    "Unable to parse StopTransaction response. Error: {:#?}",
                                    err
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else {
            self.send_message_to_web_client(
                &msg.client_id,
                &format!("improper payload:\r\n{}", &msg.payload),
                None,
            )
        }
    }
}

impl Handler<MessageFromChargeStation> for OcppServer {
    type Result = ();

    fn handle(&mut self, msg: MessageFromChargeStation, _: &mut Context<Self>) -> Self::Result {
        if let Some(call) = msg.call {
            if let Some(webclient_id) = self.chargers_webclients_pair.get(msg.charger_id.as_str()) {
                let call_as_string = format!(
                    "Call from {}:\r\n[2, \"{}\", \"{}\", {}]",
                    call.unique_id,
                    msg.charger_id,
                    call.action,
                    call.payload.as_str().unwrap()
                );
                self.send_message_to_web_client(webclient_id, &call_as_string, None);
            }
        }
        if let Some(call_error) = msg.call_error {
            let key = call_error.unique_id.clone().replace("\"", "");
            if let Some(webclient_id) = self.awaiting_call_result.get(&key) {
                let call_error_as_a_string = format!(
                    "Call error from {}:\r\n[4, \"{}\", \"{}\", \"{}\", {}]",
                    msg.charger_id,
                    call_error.unique_id,
                    call_error.error_code,
                    call_error.error_description,
                    call_error.error_details
                );
                self.send_message_to_web_client(webclient_id, &call_error_as_a_string, None);
                self.awaiting_call_result
                    .remove(call_error.unique_id.as_str());
            }
        }
        if let Some(call_result) = msg.call_result {
            let key = call_result.unique_id.clone().replace("\"", "");
            if let Some(webclient_id) = self.awaiting_call_result.get(&key) {
                let call_result_as_a_string = format!(
                    "Call result from {}: \r\n{}",
                    msg.charger_id,
                    wrap_call_result(&call_result.unique_id, (&call_result.payload).to_string())
                );
                self.send_message_to_web_client(webclient_id, &call_result_as_a_string, None);
                self.awaiting_call_result
                    .remove(call_result.unique_id.as_str());
            }
        }
    }
}
