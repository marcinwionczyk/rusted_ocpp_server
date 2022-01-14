use crate::json_rpc::*;
use crate::logs;
use crate::messages::*;
use crate::server;
use crate::server::{ConnectWebClient, DisconnectWebClient, MessageToWebBrowser};
use actix::prelude::*;
use log::info;
use uuid::Uuid;
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use chrono::{DateTime, Local, SecondsFormat, TimeZone};
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
        let mut r = JsonRpcResponse::default();
        r.result = Value::from(msg.message);
        r.id = Value::from(Uuid::new_v4().to_string());
        ctx.text(r.dump())
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebBrowserWebSocketSession {
    fn handle(&mut self, msg_result: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(msg) = msg_result {
            match msg {
                ws::Message::Ping(msg) => {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                }
                ws::Message::Pong(_) => {
                    self.hb = Instant::now();
                }
                ws::Message::Text(text) => {
                    let mut r = JsonRpcResponse::default();
                    if let Ok(json_rpc_request) =
                        serde_json::from_str::<JsonRpcRequest>(text.as_str())
                    {
                        r.id = json_rpc_request.id;
                        match json_rpc_request.method.to_lowercase().as_str() {
                            "connect" => {
                                r.result = Value::from("connected to the ocpp server");
                                ctx.text(r.dump())
                            }
                            "disconnect" => {
                                r.result = Value::from("disconnecting from the ocpp server");
                                self.address.do_send(DisconnectWebClient {
                                    serial_id: self.id.clone(),
                                });
                                ctx.text(r.dump())
                            }
                            "get_current_timestamp" => {
                                r.result = Value::from(
                                    Local::now().to_rfc3339_opts(SecondsFormat::Millis, false),
                                );
                                ctx.text(r.dump())
                            }
                            "get_log" => {
                                dotenv::from_filename("settings.env").ok();
                                let config = crate::config::Config::from_env().unwrap();
                                let mut charger_sn = "";
                                let mut begin_timestamp: DateTime<Local> =
                                    chrono::Local.ymd(1970, 1, 1).and_hms(1, 0, 0);
                                let mut end_timestamp: DateTime<Local> =
                                    chrono::offset::Local::now();
                                if let Some(params) = json_rpc_request.params {
                                    if let Some(charger_sn_value) = params.get("charger_sn")
                                    {
                                        if let Some(charger_sn_str_from_value) =
                                        charger_sn_value.as_str()
                                        {
                                            charger_sn = charger_sn_str_from_value;
                                        }
                                    } else {
                                        r.error = Some(RpcErrorData::std(-32602));
                                        ctx.text(r.dump())
                                    }
                                    if let Some(begin_timestamp_value) = params.get("begin_timestamp")
                                    {
                                        if let Ok(datetime_as_str) = DateTime::parse_from_rfc3339(
                                            begin_timestamp_value.as_str().unwrap(),
                                        ) {
                                            begin_timestamp = DateTime::from(datetime_as_str);
                                        } else {
                                            r.error = Some(RpcErrorData::std(-32602));
                                            ctx.text(r.dump())
                                        }
                                    }
                                    if let Some(end_timestamp_value) = params.get("end_timestamp"){
                                        if let Ok(datetime_as_str) = DateTime::parse_from_rfc3339(end_timestamp_value.as_str().unwrap()){
                                            end_timestamp = DateTime::from(datetime_as_str);
                                        } else {
                                            r.error = Some(RpcErrorData::std(-32602));
                                            ctx.text(r.dump())
                                        }
                                    }
                                    match logs::get_logs(&self.db_connection, charger_sn, begin_timestamp, end_timestamp){
                                        Ok(filename) => {
                                            r.result = Value::from(format!("{{\"address\":\"{}://{}:{}/logs/{}\"}}",
                                                                           if config.server.use_tls { "https" } else { "http" }, config.server.host, config.server.port, filename));
                                            ctx.text(r.dump())
                                        }
                                        Err(e) => {
                                            r.error = Some(RpcErrorData{
                                                code: -32603,
                                                message: "Internal error".to_string(),
                                                data: Value::from(format!("Unable to get logs from database. Reason: {:#?}", e))
                                            });
                                            ctx.text(r.dump())

                                        }
                                    }
                                }
                            }
                            "clear_logs" => {
                                if let Err(e) = logs::clear_logs(&self.db_connection){
                                    r.error = Some(RpcErrorData{
                                        code: -32603,
                                        message: "Internal error".to_string(),
                                        data: Value::from(format!("Unable to delete logs from database. Reason: {:#?}",
                                                          e))
                                    });
                                    ctx.text(r.dump())
                                } else {
                                    r.result = Value::from("logs cleared");
                                    ctx.text(r.dump())
                                }
                            }
                            _ => {
                                r.error = Some(RpcErrorData::std(-32601));
                                ctx.text(r.dump())
                            }
                        }
                    } else {
                        info!("websocket message not parsed: {}", text);
                        r.error = Some(RpcErrorData::std(-32700));
                        ctx.text(r.dump())
                    }
                }
                _ => {}
            }
        } else {
            ctx.stop();
            return;
        }
    }
}
