use serde_json::json;

use crate::{models::{basket::BasketOverview, market_context::MarketContext}, errors::ReweError, paths::ApiPath};

async fn fetch_basket_overview(
    basket_id: &str
) -> Result<BasketOverview, ReweError> {
    ApiPath::BasketOverview.get(&[("basketId", basket_id)], None).await
}

async fn create_basket(
    market_ctx: &MarketContext
) -> Result<BasketOverview, ReweError> {
    ApiPath::BasketOverview.get(&[], Some(market_ctx.headers())).await
}

pub enum ModifyBasketAction {
    RemoveItem,
    ModifyQuantity { quantity: u8 }
}

impl ModifyBasketAction {
    fn get_action(&self, listing_id: &str) -> serde_json::Value{
        match self {
            ModifyBasketAction::RemoveItem => json!({
                "action": "deleteLineItem",
                "listingId": listing_id
            }),
            ModifyBasketAction::ModifyQuantity { quantity } => json!({
                "action": "modifyLineItem",
                "lineItem": {
                    "listing": {
                        "id": listing_id
                    },
                    "quantity": quantity
                }
            })
        }
    }
}

async fn modify_basket(
    basket_id: &str,
    action: ModifyBasketAction,
    product_listing_id: &str,
    market_ctx: &MarketContext
) -> Result<BasketOverview, ReweError>{

    let body = json!({
        "version": 0,
        "updates": [action.get_action(product_listing_id)]
    });
    ApiPath::BasketOverview.post(&[
        ("marketCode", market_ctx.market_id.to_string().as_str()),
        ("serviceTypes", market_ctx.service_types.to_str()),
        ("basketId", basket_id)
    ], Some(market_ctx.headers()), &body).await
}

/// Create and manage your basket
pub struct ReweBasket {
    pub content: BasketOverview,
    ctx: MarketContext
}

impl ReweBasket {
    /// Get the basket by its id
    pub async fn from_id(id: &str, market_ctx: &MarketContext) -> Result<ReweBasket, ReweError> {
        Ok(ReweBasket {
            content: fetch_basket_overview(id).await?,
            ctx: market_ctx.clone()
        })
    }

    /// Create a new basket in a market
    pub async fn new(market_ctx: &MarketContext) -> Result<ReweBasket, ReweError> {
        let basket = create_basket(market_ctx).await?;
        Ok(ReweBasket {
            content: basket,
            ctx: market_ctx.clone()
        })
    }
    /// Add, increment or decrement a product quantity in the basket
    pub async fn change_product_quantity(&mut self, product_listing_id: &str, quantity: u8) -> Result<(), ReweError> {
        if quantity == 0 {
            self.remove_product(product_listing_id).await?;
        } else {
            self.content = modify_basket(self.id(), ModifyBasketAction::ModifyQuantity { quantity }, product_listing_id, &self.ctx).await?;
        }
        Ok(())
    }

    
    pub async fn remove_product(&mut self, product_listing_id: &str) -> Result<(), ReweError> {
        self.content = modify_basket(self.id(), ModifyBasketAction::RemoveItem, product_listing_id, &self.ctx).await?;
        Ok(())
    }

    pub fn id(&self) -> &str {
        &self.content.id
    }
}