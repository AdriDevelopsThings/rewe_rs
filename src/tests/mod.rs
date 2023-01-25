#![doc = include_str!("README.md")]

use crate::models::{market_context::MarketContext, market::MarketService};

pub const TEST_MARKET_ID: u32 = 433232;
pub const TEST_ZIP_CODE: &str = "99089";
pub const TEST_SEARCH_TERM: &str = "club";
pub const TEST_PRODUCT_LISTING_ID: &str = "13-4011200652902-af20ec67-0563-3116-87f7-7fcfd63d1c70";

pub fn default_test_ctx() -> MarketContext {
    MarketContext { market_id: TEST_MARKET_ID, zip_code: String::from(TEST_ZIP_CODE), service_types: MarketService::Pickup }
}

mod market;
mod product;
mod basket;
mod offer;