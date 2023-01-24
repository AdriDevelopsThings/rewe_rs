use crate::market::details::get_market_details_by_id;
use crate::market::services::get_shop_services;
use crate::tests::{TEST_MARKET_ID, TEST_ZIP_CODE};

#[tokio::test]
pub async fn test_market_details() {
    get_market_details_by_id(TEST_MARKET_ID).await.unwrap();
}

#[tokio::test]
pub async fn test_shop_services() {
    get_shop_services(TEST_ZIP_CODE).await.unwrap();
}