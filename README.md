# rewe_rs
The (mobile) api seems very difficult for me so I built a rust sdk.

## Installation
Add this crate to your project by adding `rewe` to your `Cargo.toml` dependencies.

## Market
### Select your market 
Go to [rewe.de](https://rewe.de) and select the button *Choose market* in the top right corner. After you choose your market you will be redirect to a URL that looks like this: `https://www.rewe.de/marktseite/erfurt-rieth/2900003/rewe-markt-mainzer-str-39/`. The part between your city and the street (in this example `2900003`) is your market id. You can build a market context now:

```rust
use rewe::MarketContext;

let ctx = MarketContext::new(market_id, zip_code);
// in this example:
let ctx = MarketContext::new(2900003, "99089");
```

You will need this context for lots of things.

## Searching for a market by a zip code
You can use the Rewe api to search for markets:
```rust
use rewe::market::services::fetch_shop_services;

let services = get_shop_services(zip_code).await?;
// services.pickup_markets is a vector of markets that have been found
```

### Getting details for a market

```rust
use rewe::market::details::get_market_details;

let details = get_market_details(ctx).await?; // use the ctx we've created earlier
```

## Products

### Search for products
```rust
use rewe::product::search::search_products;
use rewe::models::sorting::Sorting;

let results = search_products(search_term, page, sorting, objects_per_page, ctx).await?;
// example:
let results = search_products("club mate", 1, Sorting::default(), 20, ctx).await?;
```

### Get details for a product
Get the `listing_id` by searching for the product.

```rust
use rewe::product::details::get_product_details;

let details = get_product_details(listing_id, ctx).await?;
```

## Basket
Create and manager your basket

### Create a new basket
```rust
use rewe::market::basket::ReweBasket;

// the basket must be mutable if you want to change it's content
let mut basket = ReweBasket::new(ctx)?.await;
```

### Add a product or change its quantity
```rust
basket.change_product_quantity(listing_id, quantity).await?;
```

### Remove a product from the basket
```rust
basket.remove_product(listing_id).await?;
```


## Unitests

Run ``cargo test`` in this repository to test it. You can change some test constances in the `src/tests/mod.rs` file.