#![doc = include_str!("README.md")]

use std::env;

use crate::models::{market_context::MarketContext, market::MarketService};
pub fn test_market_id() -> u32 {
    env::var("TEST_MARKET_ID").unwrap_or_else(|_| String::from("433232")).parse().unwrap()
}

pub fn test_zip_code() -> String {
    env::var("TEST_ZIP_CODE").unwrap_or_else(|_| String::from("99089"))
}

pub fn test_search_term() -> String {
    env::var("TEST_SEARCH_TERM").unwrap_or_else(|_| String::from("club"))
}

pub fn test_product_listing_id() -> String {
    env::var("TEST_PRODUCT_LISTING_ID").unwrap_or_else(|_| String::from("13-4029764001807-af20ec67-0563-3116-87f7-7fcfd63d1c70"))
}

pub fn default_test_ctx() -> MarketContext {
    MarketContext { market_id: test_market_id(), zip_code: test_zip_code(), service_types: MarketService::Pickup }
}

mod market;
mod product;
mod basket;
mod offer;