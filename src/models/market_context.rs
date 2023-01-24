use reqwest::header::HeaderMap;

use super::market::MarketService;

/// Use the MarketContext to build a simple context for the markt you selected, because the Rewe api needs this data.
#[derive(Clone)]
pub struct MarketContext {
    pub market_id: u32,
    pub zip_code: String,
    pub service_types: MarketService
}

impl MarketContext {
    pub fn headers(&self) -> HeaderMap{
        let mut headers = HeaderMap::new();
        headers.insert("X-Rd-Market-Id", self.market_id.to_string().parse().unwrap());
        headers.insert("X-Rd-Customer-Zip", self.zip_code.parse().unwrap());
        headers.insert("X-Rd-Service-Types", self.service_types.to_str().parse().unwrap());
        headers
    }

    /// I actually don't know why you need a zip code, but it's a requirement of the Rewe mobile api
    /// ServiceTypes's default is Pickup (because you mostly need this type I guess)
    pub fn new(market_id: u32, zip_code: String) -> Self {
        Self { market_id, zip_code, service_types: MarketService::Pickup }
    }
}