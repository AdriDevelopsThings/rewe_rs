use crate::market::details::get_market_details_by_id;
use crate::market::services::get_shop_services;
use crate::tests::{test_zip_code, test_market_id};

#[tokio::test]
pub async fn test_market_details() {
    get_market_details_by_id(test_market_id()).await.unwrap();
}

#[tokio::test]
pub async fn test_shop_services() {
    get_shop_services(&test_zip_code()).await.unwrap();
}