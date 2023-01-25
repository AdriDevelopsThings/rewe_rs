//! # Introduction
//! You have to create a [`MarketContext`] to select a market
//! # Api paths
//! - Market: [`market`]
//! - Products: [`product`]
//! - Basket: [`market::basket::ReweBasket`]
//! - Offers: [`offer`]
 
#[cfg(test)]
mod tests;

pub mod errors;
pub mod models;
mod paths;

pub mod market;
pub mod product;
pub mod offer;

pub use models::market_context::MarketContext;