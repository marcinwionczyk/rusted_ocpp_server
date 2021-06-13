use actix::prelude::*;
use std::collections::HashMap;
use serde::{ Serialize, Deserialize};
use serde_json::{Value, Error};
use uuid::Uuid;
use crate::messages::{requests, responses, wrap_call};
use crate::client::WebBrowserWebSocketSession;
use serde_json::value::Value::Null;
use crate::messages::requests::{CancelReservationRequest, CertificateSignedRequest};
// Code below is for handling multiple websocket sessions between Ocpp server and charge points
//                ,_____________
//                | web client  |
//                `____   ______'
//                     \ /
//                      |
//                   websocket
//                   session
//                      |
//                 _____^_______
//       ,--------| Ocpp Server |-------.
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
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageToWebBrowser(pub String);

/// a OCPP message to OCPP server from web client
#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct MessageFromWebBrowser {
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

impl actix::Message for GetChargers { type Result = Vec<String>; }

/// `OcppServer` manages websocket sessions with charge stations
pub struct OcppServer {
    websocket_workers: HashMap<String, Recipient<MessageToChargeStation>>,
}

impl OcppServer {
    pub fn new() -> OcppServer {
        OcppServer { websocket_workers: HashMap::new() }
    }

    fn send_message_to_charger(&self, charger: &String, message: &String) {
        if let Some(session) = self.websocket_workers.get(charger) {
            let _ = session.do_send(MessageToChargeStation(message.to_owned()));
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
        println!("OcppServer: Inserting: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<DisconnectCharger> for OcppServer {
    type Result = ();
    fn handle(&mut self, msg: DisconnectCharger, _: &mut Context<Self>) -> Self::Result {
        println!("OcppServer: Removing: {}", msg.serial_id);
        self.websocket_workers.remove(msg.serial_id.as_str());
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

    fn handle(&mut self, mut msg: MessageFromWebBrowser, _: &mut Context<Self>) -> Self::Result {
        println!("sending message to: {}", msg.charger);
        let message_id = Uuid::new_v4().to_simple().to_string();
        match msg.selected.as_str() {
            "CancelReservation" => {
                let res: Result<CancelReservationRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                match res {
                    Ok(payload) => {
                        let call = wrap_call(&message_id, &msg.selected, &serde_json::to_string(&payload).unwrap());
                        self.send_message_to_charger(&msg.charger, &call);
                    }
                    Err(_) => return()
                }
            }
            "CertificateSigned" => {
                let res: Result<CertificateSignedRequest, serde_json::Error> =
                    serde_json::from_value(msg.payload);
                match res {
                    Ok(payload) => {
                        let call = wrap_call(&message_id, &msg.selected, &serde_json::to_string(&payload).unwrap());
                        self.send_message_to_charger(&msg.charger, &call);
                    }
                    Err(_) => return()
                }
            }
            _ => {}
        }
    }
}

