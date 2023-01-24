use reqwest::{header::{HeaderMap, HeaderValue}, Response};
use serde::{Deserialize, Serialize};

use crate::errors::ReweError;

pub const API_BASE: &str ="https://mobile-api.rewe.de";
pub const USER_AGENT: &str = "rewe_rs";

pub enum ApiPath {
    MarketDetails,
    ShopServices,
    ProductSearch,
    ProductDetails,
    BasketOverview
}

fn error_handler(response: Response) -> Result<Response, ReweError> {
    let status = response.status().as_u16();
    if (400..=499).contains(&status) {
        return Err(ReweError::StatusCodeClientError(response));
    } else if status >= 500 {
        return Err(ReweError::StatusCodeServerError(response));
    }
    Ok(response)
}

impl ApiPath {
    pub fn to_str(&self) -> &str {
        match self {
            ApiPath::MarketDetails => "/api/v3/market/details",
            ApiPath::ShopServices => "/mobile/shop/services",
            ApiPath::ProductSearch => "/api/v3/product-search",
            ApiPath::ProductDetails => "/api/v3/product-details",
            ApiPath::BasketOverview => "/api/v3/basket-overview"
        }
    }

    fn default_header() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-Rd-Basket-Id", "".parse().unwrap());
        headers.insert("X-Rd-Customer-Zip", "".parse().unwrap());
        headers.insert("X-Rd-Service-Types", "".parse().unwrap());
        headers
    }

    pub async fn get<T>(&self, query: &[(&str, &str)], headers: Option<HeaderMap<HeaderValue>>) -> Result<T, ReweError>
    where T: for<'de> Deserialize<'de> {
        let client = reqwest::Client::new();
        let response = client.get(String::from(API_BASE) + self.to_str())
            .header("User-Agent", USER_AGENT)
            .headers(headers.unwrap_or_else(Self::default_header))
            .query(query)
            .send().await?;
        let response = error_handler(response)?;
        Ok(response.json().await?)
    }

    pub async fn post<R, T>(&self, query: &[(&str, &str)], headers: Option<HeaderMap<HeaderValue>>, body: &R) -> Result<T, ReweError>
    where R: Serialize,
        T: for<'de> Deserialize<'de> {
        let client = reqwest::Client::new();
        let response = client.post(String::from(API_BASE) + self.to_str())
            .json(body)
            .header("User-Agent", USER_AGENT)
            .headers(headers.unwrap_or_else(Self::default_header))
            .query(query)
            .send().await?;
        let response = error_handler(response)?;
        Ok(response.json().await?)
    }
}