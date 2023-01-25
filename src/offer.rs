use crate::{MarketContext, models::offer::AllOffers, errors::ReweError, paths::ApiPath};

pub async fn get_all_offers(market_ctx: MarketContext) -> Result<AllOffers, ReweError> {
    ApiPath::AllOffers.get(
        &[("marketCode", market_ctx.market_id.to_string().as_str())],
        Some(market_ctx.headers())
    ).await
}