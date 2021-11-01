use crate::logs;
use crate::messages::*;
use crate::server;
use crate::server::{ConnectWebClient, DisconnectWebClient, MessageToWebBrowser};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use chrono::{DateTime, Local, SecondsFormat, TimeZone};
use log::error;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::Value;
use std::time::Instant;

pub struct WebBrowserWebSocketSession {
    pub id: String,
    pub hb: Instant,
    pub address: Addr<server::OcppServer>,
    pub db_connection: PooledConnection<SqliteConnectionManager>,
}

impl WebBrowserWebSocketSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebBrowserWebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.address
            .send(ConnectWebClient {
                addr: addr.recipient(),
                serial_id: self.id.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with the ocpp server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.address.do_send(server::DisconnectWebClient {
            serial_id: self.id.clone(),
        });
        Running::Stop
    }
}

impl Handler<server::MessageToWebBrowser> for WebBrowserWebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: MessageToWebBrowser, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg) {
            Ok(message_to_web_browser) => ctx.text(message_to_web_browser),
            Err(_) => {}
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebBrowserWebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let json: Value =
                    serde_json::from_str(text.as_str()).expect("JSON string is wrong");
                match json.get("message") {
                    None => {}
                    Some(value) => {
                        match value.as_str().unwrap().to_lowercase().as_str() {
                            "connect" => {
                                match serde_json::to_string(&MessageToWebBrowser {
                                    message: "connected to the ocpp server".to_string(),
                                    payload: None,
                                }) {
                                    Ok(text) => ctx.text(text),
                                    Err(_) => {}
                                }
                            }
                            "disconnect" => {
                                match serde_json::to_string(&MessageToWebBrowser {
                                    message: "disconnecting from the ocpp server".to_string(),
                                    payload: None,
                                }) {
                                    Ok(text) => ctx.text(text),
                                    Err(_) => {}
                                }
                                self.address.do_send(DisconnectWebClient {
                                    serial_id: self.id.clone(),
                                });
                            }
                            "get_current_timestamp" => {
                                match serde_json::to_string(&MessageToWebBrowser {
                                    message: "current_timestamp".to_string(),
                                    payload: Some(
                                        format!(
                                            "\"{}\"",
                                            Local::now()
                                                .to_rfc3339_opts(SecondsFormat::Millis, false)
                                        )
                                        .parse()
                                        .unwrap(),
                                    ),
                                }) {
                                    Ok(text) => ctx.text(text),
                                    Err(_) => {}
                                }
                            }
                            "get_log" => {
                                dotenv::from_filename("settings.env").ok();
                                let config = crate::config::Config::from_env().unwrap();
                                match json.get("payload") {
                                    None => {
                                        ctx.text("{\"message\":\"You forgot about payload json with ('name', 'start', 'end') keys. 'end' key as optional. 'start', 'end' in rfc3339 format (for example: 1996-12-19T16:39:57-08:00) \"}");
                                    }
                                    Some(payload) => {
                                        let charger_sn = if payload.get("charger_sn").is_some() {
                                            payload.get("charger_sn").unwrap().as_str().unwrap()
                                        } else {
                                            error!("Unable to parse charger_sn. charger_sn set to \"\"");
                                            ""
                                        };

                                        let begin_timestamp: DateTime<Local> = if payload
                                            .get("begin_timestamp")
                                            .is_some()
                                        {
                                            match DateTime::parse_from_rfc3339(
                                                payload
                                                    .get("begin_timestamp")
                                                    .unwrap()
                                                    .as_str()
                                                    .unwrap(),
                                            ) {
                                                Ok(b) => DateTime::from(b),
                                                Err(e) => {
                                                    error!("Unable to parse begin_timestamp. Reason: {:#?} \r\nSetting timestamp to 1970.01.01T01:00:00", e);
                                                    chrono::Local.ymd(1970, 1, 1).and_hms(1, 0, 0)
                                                }
                                            }
                                        } else {
                                            error!("begin_timestamp not in payload. Setting timestamp to 1970.01.01T01:00:00");
                                            chrono::Local.ymd(1970, 1, 1).and_hms(1, 0, 0)
                                        };
                                        let end_timestamp_param = payload.get("end_timestamp");
                                        let end_timestamp: DateTime<Local> =
                                            if end_timestamp_param.is_some() {
                                                match DateTime::parse_from_rfc3339(
                                                    end_timestamp_param.unwrap().as_str().unwrap(),
                                                ) {
                                                    Ok(end) => DateTime::from(end),
                                                    Err(_) => chrono::offset::Local::now(),
                                                }
                                            } else {
                                                chrono::offset::Local::now()
                                            };
                                        match logs::get_logs(
                                            &self.db_connection,
                                            charger_sn,
                                            begin_timestamp,
                                            end_timestamp,
                                        ) {
                                            Ok(filename) => {
                                                ctx.text(format!("{{\"message\":\"get_log\", \"payload\":{{\"address\":\"http://{}:{}/logs/{}\"}}}}", config.server.host, config.server.port, filename));
                                            }
                                            Err(e) => {
                                                error!("Unable to get logs from database. Reason: {:#?}", e)
                                            }
                                        }
                                    }
                                };
                            }
                            "clear_log" => match logs::clear_logs(&self.db_connection) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Unable to delete logs from database. Reason: {:#?}", e)
                                }
                            },
                            _ => {
                                match serde_json::to_string(&MessageToWebBrowser {
                                    message: "unrecognized command".to_string(),
                                    payload: None,
                                }) {
                                    Ok(text) => ctx.text(text),
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
