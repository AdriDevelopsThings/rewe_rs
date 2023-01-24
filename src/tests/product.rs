use crate::{product::{search::search_products, details::get_product_details}, models::sorting::Sorting, tests::{default_test_ctx, TEST_SEARCH_TERM}};

use super::TEST_PRODUCT_LISTING_ID;

#[tokio::test]
async fn test_product_search() {
    search_products(
        TEST_SEARCH_TERM,
        1,
        Sorting::default(),
        20,
        &default_test_ctx()
    ).await.unwrap();
}

#[tokio::test]
async fn test_product_details() {
    get_product_details(
        TEST_PRODUCT_LISTING_ID,
        &default_test_ctx()
    ).await.unwrap();
}