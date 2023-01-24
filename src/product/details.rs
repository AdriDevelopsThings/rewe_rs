use crate::{models::{product::ProductDetails, market_context::MarketContext}, errors::ReweError, paths::ApiPath};

pub async fn get_product_details(
    listing_id: &str,
    market_ctx: &MarketContext
) -> Result<ProductDetails, ReweError> {
    ApiPath::ProductDetails.get(&[
        ("marketCode", market_ctx.market_id.to_string().as_str()),
        ("listingId", listing_id),
        ("serviceTypes", market_ctx.service_types.to_str())
    ], None).await
}