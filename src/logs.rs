use rusqlite::{Connection, Result, params};
use chrono::{Local, SecondsFormat};
use log::{error, warn, info, debug, trace};

pub(crate) fn create_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("logs.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS chargers (
        id INTEGER CONSTRAINT chargers_pk PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL)", [])?;
    conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS chargers_name_uindex ON chargers (name)", [])?;
    conn.execute("CREATE TABLE  IF NOT EXISTS logs (
        id INTEGER CONSTRAINT chargers_pk PRIMARY KEY AUTOINCREMENT,
        timestamp DATETIME NOT NULL,
        charger_id INTEGER REFERENCES chargers ON UPDATE CASCADE ON DELETE CASCADE,
        level TEXT DEFAULT 'info' not null,
        message TEXT)", [])?;
    Ok(())
}

pub(crate) fn add_charger(conn: &Connection, charger_name: &str) -> Result<(), rusqlite::Error> {
    let _ = conn.execute("INSERT INTO chargers (name) VALUES (?1)", params![charger_name]);
    Ok(())
}

fn add_log_priv(conn: &Connection, charger_name: &str, log_level_option: Option<String>, message: String) -> Result<()> {
    let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Millis, false);
    let charger_id: i32 = conn.query_row("SELECT id FROM chargers WHERE name = ?1;",
                                    params![charger_name],
                                    |row| row.get(0))?;

    match log_level_option{
        None => {
            conn.execute("INSERT INTO logs (timestamp, charger_id, message) VALUES (?1, ?2, ?3)",
                         &[&timestamp, &charger_id.to_string(), &message])?;
        }
        Some(log_level) => {
            conn.execute("INSERT INTO logs (timestamp, charger_id, level, message) VALUES (?1, ?2, ?3, ?4)",
                         &[&timestamp, &charger_id.to_string(), &log_level, &message])?;
        }
    }
    Ok(())
}

//select timestamp, level, message from logs left join chargers c on logs.charger_id = c.id where c.name = 'ORAC2-KR1-0001-013' and timestamp >= 1634653600000;

pub(crate) fn add_log(conn: &Connection, charger_name: &str, log_level_option: Option<String>, message: String){
    match add_log_priv(conn, charger_name.clone(), log_level_option.clone(), message.clone()){
        Ok(_) => {}
        Err(e) => {
            error!("Failed at adding log to the database. Reason: {:#?}", e);
        }
    }
    match log_level_option{
        None => {
            info!("{}: {}", charger_name, message);
        }
        Some(log_level) => {
            match log_level.as_str() {
                "error" => error!("{}: {}", charger_name, message),
                "warn" => warn!("{}: {}", charger_name, message),
                "info" => info!("{}: {}", charger_name, message),
                "debug" => debug!("{}: {}", charger_name, message),
                "trace" => trace!("{}: {}", charger_name, message),
                &_ => {}
            }
        }
    }
}