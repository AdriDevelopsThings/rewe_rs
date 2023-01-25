use crate::{product::{search::search_products, details::get_product_details}, models::sorting::Sorting, tests::{default_test_ctx, test_search_term, test_product_listing_id}};

#[tokio::test]
async fn test_product_search() {
    search_products(
        &test_search_term(),
        1,
        Sorting::default(),
        20,
        &default_test_ctx()
    ).await.unwrap();
}

#[tokio::test]
async fn test_product_details() {
    get_product_details(
        &test_product_listing_id(),
        &default_test_ctx()
    ).await.unwrap();
}