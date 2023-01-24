use serde::Deserialize;

use super::{market::MarketService, price::Price};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasketOverview {
    pub id: String,
    pub version: u8,
    pub service_type: Option<MarketService>,
    pub store_id: Option<String>,
    pub zip_code: Option<String>,
    pub not_available_products: Vec<BasketProduct>,
    pub available_products: Vec<BasketProduct>,
    pub summary: BasketSummary,
    pub violations: Vec<String>, //todo
    pub min_delivery: MinDelivery
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasketProduct {
    pub listing_id: String,
    pub product_id: String,
    pub name: String,
    pub grammage: String,
    pub refund_price: Option<u16>,
    pub deposit_label: Option<String>,
    pub image_url: String,
    pub item_count: u8,
    pub order_limit: u8,
    pub bulky_goods_share: u8,
    pub single_price: Price,
    pub total_price: Price,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasketSummary {
    pub item_count: u8,
    pub price_items: u32,
    pub price_deposit: u32,
    pub price_beverage_surcharge: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinDelivery {
    pub price: Option<u32>,
    pub minimum_order_amount: Option<u8>
}