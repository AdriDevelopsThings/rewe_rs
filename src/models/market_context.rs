use reqwest::header::HeaderMap;

use super::market::MarketService;

/// Use the MarketContext to build a simple context for the markt you selected, because the Rewe api needs this data.
/// Go to [rewe.de](https://rewe.de) and select the button *Choose market* in the top right corner. After you choose your market you will be redirect to a URL that looks like this: `https://www.rewe.de/marktseite/erfurt-rieth/2900003/rewe-markt-mainzer-str-39/`. The part between your city and the street (in this example `2900003`) is your market id. You can build a market context now:
/// ```
/// let ctx = MarketContext::new(market_id, zip_code);
/// // in this example:
/// let ctx = MarketContext::new(2900003, "99089");
/// ```
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