use crate::{models::market::ShopServices, errors::ReweError, paths::ApiPath};

pub async fn get_shop_services(zip_code: &str) -> Result<ShopServices, ReweError> {
    ApiPath::ShopServices.get(&[("zipCode", zip_code)], None).await
}