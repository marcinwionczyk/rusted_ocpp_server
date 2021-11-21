use std::ffi::OsStr;
use chrono::{DateTime, Local, SecondsFormat};
use log::{debug, error, info, trace, warn};
use rusqlite::{params, Connection, Result};
use std::fs::{File, remove_file, read_dir};
use std::io::Write;

struct LogsReturned {
    timestamp: String,
    message: String,
}

pub fn create_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("logs.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chargers (
        id INTEGER CONSTRAINT chargers_pk PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL)",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS chargers_name_uindex ON chargers (name)",
        [],
    )?;
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS logs (
        id INTEGER CONSTRAINT logs_pk PRIMARY KEY AUTOINCREMENT,
        timestamp DATETIME NOT NULL,
        charger_id INTEGER REFERENCES chargers ON UPDATE CASCADE ON DELETE CASCADE,
        level TEXT DEFAULT 'info' not null,
        message TEXT)",
        [],
    )?;
    Ok(())
}

pub fn add_charger(conn: &Connection, charger_name: &str) -> Result<(), rusqlite::Error> {
    let _ = conn.execute(
        "INSERT INTO chargers (name) VALUES (?1)",
        params![charger_name],
    );
    Ok(())
}

fn add_log_priv(
    conn: &Connection,
    charger_name: &str,
    log_level_option: Option<String>,
    message: String,
) -> Result<()> {
    let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Millis, false);
    let charger_id: i32 = conn.query_row(
        "SELECT id FROM chargers WHERE name = ?1;",
        params![charger_name],
        |row| row.get(0),
    )?;

    match log_level_option {
        None => {
            conn.execute(
                "INSERT INTO logs (timestamp, charger_id, message) VALUES (?1, ?2, ?3)",
                &[&timestamp, &charger_id.to_string(), &message],
            )?;
        }
        Some(log_level) => {
            conn.execute(
                "INSERT INTO logs (timestamp, charger_id, level, message) VALUES \
            (?1, ?2, ?3, ?4)",
                &[&timestamp, &charger_id.to_string(), &log_level, &message],
            )?;
        }
    }
    Ok(())
}

pub fn add_log(
    conn: &Connection,
    charger_name: &str,
    log_level_option: Option<String>,
    message: String,
) {
    if let Err(e) = add_log_priv(
        conn,
        charger_name.clone(),
        log_level_option.clone(),
        message.clone(),
    ) {
        error!("Failed at adding log to the database. Reason: {:#?}", e);
    }
    match log_level_option {
        None => {
            info!("{}: {}", charger_name, message);
        }
        Some(log_level) => match log_level.as_str() {
            "error" => error!("{}: {}", charger_name, message),
            "warn" => warn!("{}: {}", charger_name, message),
            "info" => info!("{}: {}", charger_name, message),
            "debug" => debug!("{}: {}", charger_name, message),
            "trace" => trace!("{}: {}", charger_name, message),
            &_ => {}
        },
    }
}

pub fn get_logs(
    conn: &Connection,
    charger_name: &str,
    start_timestamp: DateTime<Local>,
    end_timestamp: DateTime<Local>,
) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT logs.timestamp, \
    logs.message FROM logs left join chargers on logs.charger_id=chargers.id WHERE \
    chargers.name=:charger_name AND (logs.timestamp BETWEEN :start_timestamp AND :end_timestamp) \
    ORDER BY logs.timestamp",
    )?;
    let filename = format!(
        "{}_{}.log",
        charger_name.clone(),
        start_timestamp.clone().timestamp()
    );
    let mut file = File::create(format!("./logs/{}", filename)).expect("Cannot create file");
    let logs_iter = stmt.query_map(
        &[
            (":charger_name", charger_name),
            (":start_timestamp", &start_timestamp.to_rfc3339()),
            (":end_timestamp", &end_timestamp.to_rfc3339()),
        ],
        |row| {
            Ok(LogsReturned {
                timestamp: row.get(0)?,
                message: row.get(1)?,
            })
        },
    )?;
    for log_result in logs_iter {
        match log_result{
            Ok(log) => {
                writeln!(file, "[{}] {}", log.timestamp, log.message).expect("Cannot write to file.");
            }
            Err(e) => {
                error!("Unable to parse log line from logs database into LogsReturned struct. Reason: {:#?}", e);
            }
        }
    }
    Ok(filename)
}

pub fn clear_logs(conn: &Connection) -> Result<usize> {
    if let Ok(entries) =  read_dir("./logs") {
        for entry in entries{
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.path().file_name() {
                    if file_name != OsStr::new("server.log"){
                        if let Err(e) = remove_file(entry.path()){
                            error!("Unable to remove file {}. Reason: {:#?}",
                                file_name.to_str().unwrap(), e);
                        }
                    } else {
                        match File::open(entry.path()){
                            Ok(mut buffer) => {
                                if let Err(e) = buffer.write_all(b""){
                                    error!("Unable to clear ./logs/server.log file. Reason: {:#?}", e);
                                }
                            }
                            Err(e) => {error!("Unable to open ./logs/server.log file. Reason: {:#?}", e)}
                        }
                    }
                }
            }
        }
    }
    conn.execute("DELETE FROM logs where timestamp < DATETIME('now', '-1 month')", [])
}