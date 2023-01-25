use crate::{models::{sorting::Sorting, product::ProductSearch, market_context::MarketContext}, errors::ReweError, paths::ApiPath};


/// Search for products by search term like this
/// ```
/// let results = search_products(search_term, page, sorting, objects_per_page, ctx).await?;
/// let results = search_products("club mate", 1, Sorting::default(), 20, ctx).await?;
/// ```
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