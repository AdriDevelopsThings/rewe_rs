use crate::{market::basket::ReweBasket, tests::{TEST_PRODUCT_LISTING_ID, default_test_ctx}, product::details::get_product_details};

#[tokio::test]
async fn test_basket() {
    let ctx = default_test_ctx();

    let mut basket = ReweBasket::new(&ctx).await.unwrap();
    basket.change_product_quantity(TEST_PRODUCT_LISTING_ID, 3).await.unwrap();
    let product = get_product_details(TEST_PRODUCT_LISTING_ID, &ctx).await.unwrap();
    let product = basket.content.available_products.iter().find(|i| i.product_id == product.product_id).unwrap();
    assert_eq!(product.item_count, 3)
}