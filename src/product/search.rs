use crate::{models::{sorting::Sorting, product::ProductSearch, market_context::MarketContext}, errors::ReweError, paths::ApiPath};

pub async fn search_products(
    search_term: &str,
    page: u8,
    sorting: Sorting,
    objects_per_page: u8,
    market_ctx: &MarketContext,
) -> Result<ProductSearch, ReweError> {
    ApiPath::ProductSearch.get(&[
        ("searchTerm", search_term),
        ("page", page.to_string().as_str()),
        ("sorting", sorting.to_str()),
        ("objectsPerPage", objects_per_page.to_string().as_str()),
        ("marketCode", market_ctx.market_id.to_string().as_str()),
        ("serviceTypes", market_ctx.service_types.to_str())
    ], Some(market_ctx.headers())).await
}