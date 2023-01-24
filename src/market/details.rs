use crate::{paths::ApiPath, errors::ReweError, models::{market::MarketDetails, market_context::MarketContext}};

pub async fn get_market_details_by_id(market_id: u32) -> Result<MarketDetails, ReweError> {
    ApiPath::MarketDetails.get(&[("marketId", market_id.to_string().as_str())], None).await
}

pub async fn get_market_details(market_ctx: MarketContext) -> Result<MarketDetails, ReweError> {
    get_market_details_by_id(market_ctx.market_id).await
}