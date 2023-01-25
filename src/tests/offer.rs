use crate::{offer::get_all_offers, tests::default_test_ctx};

#[tokio::test]
pub async fn test_all_offers() {
    get_all_offers(default_test_ctx()).await.unwrap();
}