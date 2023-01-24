use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub postal_code: String,
    pub street: String,
    pub city: String,
    pub house_number: String
}