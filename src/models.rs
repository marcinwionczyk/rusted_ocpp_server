use serde::{Deserialize, Serialize};
use super::schema::available_chargers;

#[derive(Queryable, Serialize)]
pub struct AvailableCharger {
    pub serial_id: String,
    pub ip_address: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "available_chargers"]
pub struct NewAvailableCharger<'a> {
    pub serial_id: &'a str,
    pub ip_address: &'a str,
}

