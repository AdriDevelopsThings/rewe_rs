use serde::Deserialize;

use super::{location::Location, address::Address, type_wrapped::TypeWrapped};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketDetails {
    pub market_item: MarketItem,
    pub phone: String,
    pub rating_url: String,
    #[serde(rename = "isLSFK")]
    pub is_lsfk: bool,
    pub opening_times: Vec<OpeningTime>,
    pub special_opening_times: Vec<String>, // todo
    pub services: Vec<TypeWrapped<MarketService>>,
    pub feature_categories: Vec<String>, // todo
    pub actions: Vec<MarketAction>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketItem {
    pub id: String,
    pub name: String,
    pub type_id: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub opening_info: String,
    pub opening_info_prefix: String,
    pub opening_type: String,
    pub feature_types: Vec<String>, // todo
    pub location: Location,
    pub raw_values: MarketItemRawValues,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketItemRawValues {
    pub attributes: Vec<String>, // todo
    pub postal_code: String,
    pub city: String
}

#[derive(Deserialize, Debug)]
pub struct OpeningTime {
    pub days: String,
    pub hours: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketService {
    Pickup,
    Parcel
}

impl MarketService {
    pub fn to_str(&self) -> &str {
        match self {
            MarketService::Pickup => "PICKUP",
            MarketService::Parcel => "PARCEL"
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MarketAction {
    #[serde(rename = "type")]
    pub action_type: MarketActionType,
    pub title: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketActionType {
    Route,
    Call,
    Rate
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShopServices {
    pub zip_code: String,
    pub city: String,
    pub shop_service: ShopService,
    pub pickup_markets: Vec<PickupMarket>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShopService {
    pub title: String,
    pub service_types: Vec<MarketService>,
    pub customer_zip_code: String,
    pub is_lsfk: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickupMarket {
    #[serde(rename = "type")]
    pub market_type: String,
    pub name: String,
    pub company: String,
    pub address: Address,
    pub geo_location: Location,

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickupMarketShopService {
    pub title: String,
    pub time_slot_title: String,
    pub service_types: Vec<MarketService>,
    pub customer_zip_code: String,
    pub market_zip_code: String,
    pub market_code: String,
    pub is_lsfk: bool,
    pub is_pickup_station: bool
}