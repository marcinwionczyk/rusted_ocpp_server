use rusqlite::{Connection, Result, params};
use chrono::{Local, SecondsFormat};

pub(crate) fn create_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("logs.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS chargers (
        id INTEGER CONSTRAINT chargers_pk PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL)", [])?;
    conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS chargers_name_uindex ON chargers (name)", [])?;
    conn.execute("CREATE TABLE  IF NOT EXISTS logs (
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

pub(crate) fn add_log(conn: &Connection, charger_name: &str, log_level_option: Option<String>, message: String) -> Result<()> {
    let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Millis, false);
    let charger_id: String = conn.query_row("SELECT id FROM chargers WHERE name = ?1;",
                                    params![charger_name],
                                    |row| row.get(0))?;

    match log_level_option{
        None => {
            conn.execute("INSERT INTO logs (timestamp, charger_id, message) VALUES (?1, ?2, ?3)",
                         &[&timestamp, &charger_id, &message])?;
        }
        Some(log_level) => {
            conn.execute("INSERT INTO logs (timestamp, charger_id, level, message) VALUES (?1, ?2, ?3, ?4)",
                         &[&timestamp, &charger_id, &log_level, &message])?;
        }
    }
    Ok(())
}
