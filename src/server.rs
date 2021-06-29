use actix::prelude::*;
use std::collections::HashMap;
use serde::{ Serialize, Deserialize};
use serde_json::{Value};
use uuid::Uuid;
use crate::messages::wrap_call;
use crate::messages;
// Code below is for handling multiple websocket sessions between Ocpp server and charge points
//                ,_____________
//                | web client  |
//                `____   ______'
//                     \ /
//                      |
//                   websocket
//                   worker
//                      |
//                 _____^_______
//                |    ocpp     |
//       ,--------|   server    |-------.
//      |         ``````|```````        |
//   websocket       websocket       websocket
//   worker          worker          worker
//      |               |               |
// .----^-------.  .----^-------.  .----^-------.
//|charge_point | |charge_point | |charge_point |
//`-------------' `-------------' `-------------'

/// Ocpp server sends this message through websocket session to the charger
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageToChargeStation(pub String);

/// Ocpp server sends this messages through websocket session to the web browser
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct MessageToWebBrowser{
    pub message: String
}

/// a OCPP message to OCPP server from web client
#[derive(Message, Clone, Deserialize)]
#[rtype(result = "()")]
pub struct MessageFromWebBrowser {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub charger: String, // target Charge point
    pub selected: String,
    pub payload: Value, // OCPP message
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
    pub serial_id: String
}

impl actix::Message for GetChargers { type Result = Vec<String>; }

/// `OcppServer` manages websocket sessions with charge stations
pub struct OcppServer {
    websocket_workers: HashMap<String, Recipient<MessageToChargeStation>>,
    webclient_workers: HashMap<String, Recipient<MessageToWebBrowser>>
}

impl OcppServer {
    pub fn new() -> OcppServer {
        OcppServer { websocket_workers: HashMap::new(), webclient_workers: HashMap::new() }
    }

    fn send_message_to_charger(&self, charger: &String, message: &String) {
        if let Some(session) = self.websocket_workers.get(charger) {
            match session.do_send(MessageToChargeStation(message.to_owned())) {
                Err(e) => {println!("{}", e.to_string())}
                Ok(_) => {}
            }
        }
    }

    fn send_message_to_web_client(&self, web_client: &String, message: &String) {
        if let Some(session) = self.webclient_workers.get(web_client) {
            match session.do_send(MessageToWebBrowser{message: message.to_owned()}) {
                Err(e) => {println!("{}", e.to_string())}
                Ok(_) => {}
            }
        }
    }

    fn message_from_web_browser_is_valid(msg: MessageFromWebBrowser) -> bool {
        match msg.selected.as_str() {
            "CancelReservation" => {
                let res: Result<messages::requests::CancelReservationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "CertificateSigned" => {
                let res: Result<messages::requests::CertificateSignedRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ChangeAvailability" => {
                let res: Result<messages::requests::ChangeAvailabilityRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ClearCache" => {
                let res: Result<messages::requests::ClearCacheRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ClearChargingProfile" => {
                let res: Result<messages::requests::ClearChargingProfileRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ClearDisplayMessage" => {
                let res: Result<messages::requests::ClearDisplayMessageRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ClearVariableMonitoring" => {
                let res: Result<messages::requests::ClearVariableMonitoringRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "CostUpdated" => {
                let res: Result<messages::requests::CostUpdatedRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "CustomerInformation" => {
                let res: Result<messages::requests::CustomerInformationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "DataTransfer" => {
                let res: Result<messages::requests::DataTransferRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "DeleteCertificate" => {
                let res: Result<messages::requests::DeleteCertificateRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetBaseReport" => {
                let res: Result<messages::requests::GetBaseReportRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetChargingProfiles" => {
                let res: Result<messages::requests::GetChargingProfilesRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetCompositeSchedule" => {
                let res: Result<messages::requests::GetCompositeScheduleRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetDisplayMessages" => {
                let res: Result<messages::requests::GetDisplayMessagesRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetInstalledCertificateIds" => {
                let res: Result<messages::requests::GetInstalledCertificateIdsRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetLocalListVersion" => {
                let res: Result<messages::requests::GetLocalListVersionRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetLog" => {
                let res: Result<messages::requests::GetLogRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetMonitoringReport" => {
                let res: Result<messages::requests::GetMonitoringReportRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetReport" => {
                let res: Result<messages::requests::GetReportRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetTransactionStatus" => {
                let res: Result<messages::requests::GetTransactionStatusRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "GetVariables" => {
                let res: Result<messages::requests::GetVariablesRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "InstallCertificate" => {
                let res: Result<messages::requests::InstallCertificateRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "PublishFirmware" => {
                let res: Result<messages::requests::PublishFirmwareRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "ReserveNow" => {
                let res: Result<messages::requests::ReserveNowRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "Reset" => {
                let res: Result<messages::requests::ResetRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SendLocalList" => {
                let res: Result<messages::requests::SendLocalListRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetChargingProfile" => {
                let res: Result<messages::requests::SetChargingProfileRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetDisplayMessage" => {
                let res: Result<messages::requests::SetDisplayMessageRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetMonitoringBase" => {
                let res: Result<messages::requests::SetMonitoringBaseRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetMonitoringLevel" => {
                let res: Result<messages::requests::SetMonitoringLevelRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetNetworkProfile" => {
                let res: Result<messages::requests::SetNetworkProfileRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetVariableMonitoring" => {
                let res: Result<messages::requests::SetVariableMonitoringRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SetVariables" => {
                let res: Result<messages::requests::SetVariablesRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "SignCertificate" => {
                let res: Result<messages::requests::SignCertificateRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "TriggerMessage" => {
                let res: Result<messages::requests::TriggerMessageRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "UnlockConnector" => {
                let res: Result<messages::requests::UnlockConnectorRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "UnpublishFirmware" => {
                let res: Result<messages::requests::UnpublishFirmwareRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            },
            "UpdateFirmware" => {
                let res: Result<messages::requests::UpdateFirmwareRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                res.is_ok()
            }
            &_ => false
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
        self.websocket_workers.insert(msg.serial_id.clone(), msg.addr);
        println!("OcppServer: Inserting charger: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<ConnectWebClient> for OcppServer {
    type Result = String;

    fn handle(&mut self, msg: ConnectWebClient, _: &mut Context<Self>) -> Self::Result {
        self.webclient_workers.insert(msg.serial_id.clone(), msg.addr);
        println!("OcppServer: Inserting web client: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<DisconnectCharger> for OcppServer {
    type Result = ();
    fn handle(&mut self, msg: DisconnectCharger, _: &mut Context<Self>) -> Self::Result {
        println!("OcppServer: Removing charger: {}", msg.serial_id);
        self.websocket_workers.remove(msg.serial_id.as_str());
    }
}

impl Handler<DisconnectWebClient> for OcppServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectWebClient, _: &mut Context<Self>) -> Self::Result {
        println!("OcppServer: Removing web client: {}", msg.serial_id.to_string());
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
        println!("sending message to: {}", msg.charger);
        let message_id = Uuid::new_v4().to_string();
        if OcppServer::message_from_web_browser_is_valid(msg.clone()) {
            let call = wrap_call(&message_id, &msg.selected, &serde_json::to_string(&msg.payload).unwrap());
            self.send_message_to_charger(&msg.charger, &call);
            self.send_message_to_web_client(&msg.client_id, &format!("call sent to charger {}:\r\n{}", &msg.charger, call))
        } else {
            self.send_message_to_web_client(&msg.client_id, &format!("improper payload:\r\n{}", &msg.payload))
        }
    }
}

