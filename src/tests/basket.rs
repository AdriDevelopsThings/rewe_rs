use crate::{market::basket::ReweBasket, tests::{test_product_listing_id, default_test_ctx}, product::details::get_product_details};

#[tokio::test]
async fn test_basket() {
    let ctx = default_test_ctx();

    let mut basket = ReweBasket::new(&ctx).await.unwrap();
    basket.change_product_quantity(&test_product_listing_id(), 3).await.unwrap();
    let product = get_product_details(&test_product_listing_id(), &ctx).await.unwrap();
    let product = basket.content.available_products.iter().find(|i| i.product_id == product.product_id).unwrap();
    assert_eq!(product.item_count, 3)
}