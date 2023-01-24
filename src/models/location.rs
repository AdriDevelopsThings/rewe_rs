use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Location {
    #[serde(deserialize_with = "parse_location_from_strings")]
    pub latitude: f64,
    #[serde(deserialize_with = "parse_location_from_strings")]
    pub longitude: f64
}

fn parse_location_from_strings<'de, D>(deserializer: D) -> Result<f64, D::Error>
where D: Deserializer<'de> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("wrong type"))
    })
}