use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TypeWrapped<T> {
    #[serde(rename = "type")]
    _type: T,
}