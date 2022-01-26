extern crate yaml_rust;
use config::ConfigError;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub heartbeat_interval: i64,
    pub use_tls: bool,
    pub time_offset: i64,
    pub ocpp_auth_password: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub users: Vec<String>,
}

impl Config {
    pub fn user_allowed(&self, user: &String) -> bool {
        return self.users.contains(user);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "".to_string(),
                port: 0,
                heartbeat_interval: 0,
                use_tls: false,
                time_offset: 0,
                ocpp_auth_password: "".to_string(),
            },
            users: vec![],
        }
    }
}

impl Config {
    pub fn from_yaml(path: &str) -> Result<Self, ConfigError> {
        if let Ok(s) = fs::read_to_string(path) {
            let mut cfg = Config::default();
            if let Ok(yaml_vec) = YamlLoader::load_from_str(s.as_str()) {
                let server_key = Yaml::String(String::from("SERVER"));
                let users_key = Yaml::String(String::from("ALLOWED USERS"));
                let host_key = Yaml::String(String::from("HOST"));
                let port_key = Yaml::String(String::from("PORT"));
                let use_tls_key = Yaml::String(String::from("USE TLS"));
                let heartbeat_interval_key = Yaml::String(String::from("HEARTBEAT INTERVAL"));
                let time_offset_key = Yaml::String(String::from("TIME OFFSET"));
                let ocpp_auth_key = Yaml::String(String::from("OCPP AUTH PASSWORD"));
                if let Some(host) = yaml_vec[0].clone().into_hash().unwrap()[&server_key][0]
                    .clone()
                    .into_hash()
                    .unwrap()[&host_key]
                    .clone()
                    .into_string()
                {
                    cfg.server.host = String::from(host);
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse HOST from config file",
                    )));
                }
                if let Some(port_i64) = yaml_vec[0].clone().into_hash().unwrap()[&server_key][1]
                    .clone()
                    .into_hash()
                    .unwrap()[&port_key]
                    .clone()
                    .into_i64()
                {
                    if let Ok(port) = u32::try_from(port_i64) {
                        cfg.server.port = port;
                    } else {
                        return Err(ConfigError::Message(String::from(
                            "cannot parse PORT from i64 to u32 type",
                        )));
                    }
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse PORT from config file",
                    )));
                }
                if let Some(use_tls) = yaml_vec[0].clone().into_hash().unwrap()[&server_key][2]
                    .clone()
                    .into_hash()
                    .unwrap()[&use_tls_key]
                    .clone()
                    .into_bool()
                {
                    cfg.server.use_tls = use_tls;
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse USE TLS from config file",
                    )));
                }
                if let Some(heartbeat_interval) = yaml_vec[0].clone().into_hash().unwrap()
                    [&server_key][3]
                    .clone()
                    .into_hash()
                    .unwrap()[&heartbeat_interval_key]
                    .clone()
                    .into_i64()
                {
                    cfg.server.heartbeat_interval = heartbeat_interval;
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse HEARTBEAT INTERVAL from config file",
                    )));
                }
                if let Some(time_offset) = yaml_vec[0].clone().into_hash().unwrap()[&server_key][4]
                    .clone()
                    .into_hash()
                    .unwrap()[&time_offset_key]
                    .clone()
                    .into_i64()
                {
                    cfg.server.time_offset = time_offset;
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse TIME OFFSET from config file",
                    )));
                }
                if let Some(ocpp_auth_password) = yaml_vec[0].clone().into_hash().unwrap()
                    [&server_key][5]
                    .clone()
                    .into_hash()
                    .unwrap()[&ocpp_auth_key]
                    .clone()
                    .into_string()
                {
                    println!("{:#?}", ocpp_auth_password);
                    cfg.server.ocpp_auth_password = ocpp_auth_password;
                }
                if let Some(users) = yaml_vec[0].clone().into_hash().unwrap()[&users_key].clone().into_vec() {
                    for user in users {
                        if let Some(user_str) = user.into_string() {
                            cfg.users.push(user_str);
                        }
                    }
                } else {
                    return Err(ConfigError::Message(String::from(
                        "cannot parse ALLOWED USERS from config file",
                    )));
                }
            }
            Ok(cfg)
        } else {
            return Err(ConfigError::Message(String::from(
                "Settings File not found",
            )));
        }
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
