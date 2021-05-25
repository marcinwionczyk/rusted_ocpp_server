use actix::prelude::*;
use std::collections::HashMap;
use crate::client;
use actix::dev::MessageResponse;

// Code below is for handling multiple websocket sessions between Ocpp server and charge points
//                 _____________
//       ,--------| Ocpp Server |-------.
//      |         ``````|```````        |
//   websocket       websocket       websocket
//   session         session         session
//      |               |               |
// .----^-------.  .----^-------.  .----^-------.
//|charge_point | |charge_point | |charge_point |
//`-------------' `-------------' `-------------'


#[derive(Message)]
#[rtype(result = "()")]
pub struct WebClientMessage {
    pub serial_id: String,
    pub text: String,
}

/// Ocpp server sends this messages to websocket session
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageToChargePoint(pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<MessageToChargePoint>,
    pub serial_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub serial_id: String,
}

pub struct OcppServer {
    websocket_sessions: HashMap<String, Recipient<MessageToChargePoint>>,
}

impl OcppServer {
    pub fn new() -> OcppServer {
        OcppServer { websocket_sessions: HashMap::new() }
    }
}

impl OcppServer {
    /// Send message at given serial_id
    fn send_message(&self, serial_id: &String, message: &String) {
        if let Some(session) = self.websocket_sessions.get(serial_id) {
            let _ = session.do_send(MessageToChargePoint(message.to_owned()));
        }
    }

    fn list_chargers(&self) -> String {
        let mut serial_ids:Vec<String> = Vec::new();
        for key in self.websocket_sessions.keys(){
            serial_ids.push(key.clone());
        }
        format!("[{}]", serial_ids.join(", "))
    }
}

impl Actor for OcppServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for OcppServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.websocket_sessions.insert(msg.serial_id.clone(), msg.addr);
        println!("OcppServer: Inserting: {}", msg.serial_id);
        msg.serial_id
    }
}

impl Handler<Disconnect> for OcppServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        println!("OcppServer: Removing: {}", msg.serial_id);
        self.websocket_sessions.remove(msg.serial_id.as_str());
    }
}

impl Handler<WebClientMessage> for OcppServer {
    type Result = ();

    fn handle(&mut self, msg: WebClientMessage, _: &mut Context<Self>) -> Self::Result {
        println!("sending message to: {}", msg.serial_id);
        self.send_message(&msg.serial_id, &msg.text);
    }
}

