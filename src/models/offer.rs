use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllOffers {
    pub handout: Handout,
    pub categories: Vec<Category>,
    pub until_date: u64,
    pub has_online_offers: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Handout {
    pub width: u32,
    pub height: u32,
    pub images: Vec<Image>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original: String,
    pub thumbnail: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub title: String,
    pub offers: Vec<Offer>,
    pub order: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub cell_type: OfferCellType,
    pub overline: String,
    pub title: String,
    pub subtitle: String,
    pub images: Vec<String>,
    #[serde(rename = "biozid")]
    pub biocide: Option<bool>,
    pub price_data: PriceData,
    pub detail: OfferProductDetail,
    pub raw_values: OfferRawValues
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum OfferCellType {
    Mood,
    Default,
    Hero
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    pub price: Option<String>,
    /// The regular_price is something like "Aktion", not a real price
    pub regular_price: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OfferProductDetail {
    pub tags: Vec<String>,
    pub contents: Vec<OfferProductDetailContent>,
    #[serde(rename = "biozid")]
    pub biocide: Option<bool>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct  OfferProductDetailContent {
    pub header: String,
    pub titles: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OfferRawValues {
    pub category_title: String,
    pub price_average: f32,
    pub flyer_page: u8,
    pub nan: String
}