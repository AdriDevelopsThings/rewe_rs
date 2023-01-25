use serde::Deserialize;

use super::{hazard_warnings::HazardAndWarnings, price::Price};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSearch {
    pub products: Vec<ProductSearchItem>,
    pub total_product_count: u16,
    pub page: u8,
    pub total_pages: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSearchItem {
    pub id: String,
    pub listing_id: String,
    pub name: String,
    pub grammage: String,
    pub image_url: String,
    pub current_price: Price,
    pub has_bio_cide: bool,
    pub order_limit: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDetails {
    pub product_id: String,
    pub listing_id: String,
    pub image_url: String,
    pub title: String,
    pub current_price: Price,
    pub tags: Vec<String>,
    pub grammage: String,
    pub refund_price: Option<u16>,
    pub deposit_label: Option<String>,
    pub discount: Option<ProductDiscount>,
    pub tiers: Vec<String>, // todo
    pub product_infos: Vec<ProductInfo>,
    pub order_amount_limit: u8,
    pub hazards_and_warnings: Option<HazardAndWarnings>,
    pub recommended_products: Vec<RecommendProduct>,
    pub is_buyable: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscount {
    pub regular_price: Price,
    pub discount_rate: String,
    pub expiration: String
}

#[derive(Deserialize, Debug)]
#[serde(tag = "title", rename_all = "camelCase")]
pub enum ProductInfo {
    #[serde(rename = "Produktbeschreibung")]
    ProductDescription {
        #[serde(rename = "infoItems")]
        info_items: Vec<ProductDescriptionItem>
    },
    #[serde(rename = "Zutaten und Allergene")]
    IngredientsAndAllergens {
        #[serde(rename = "infoItems")]
        info_items: Vec<IngredientsAndAllergensItem>
    },
    #[serde(rename = "Nährwerte")]
    NutritionalValues {
        #[serde(rename = "nutritionTables")]
        nutrition_tables: Vec<NutritionTable>,
    },
    #[serde(rename = "Artikeldetails")]
    ArticleInformation {
        #[serde(rename = "infoItems")]
        info_items: Vec<GeneralInformationItem>
    },
    #[serde(rename = "Hinweise")]
    Notes {
        #[serde(rename = "infoItems")]
        info_items: Vec<GeneralInformationItem>
    },
    #[serde(rename = "Inhaltsstoffe")]
    Ingredients {
        #[serde(rename = "infoItems")]
        info_items: Vec<GeneralInformationItem>
    },
    #[serde(rename = "Kontakt")]
    Contact {
        #[serde(rename = "infoItems")]
        info_items: Vec<ContactInformationItem>
    },
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug)]
pub struct ProductDescriptionItem {
    pub title: String,
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct IngredientsAndAllergensItem {
    pub description: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NutritionTable {
    pub title: String,
    pub nutrition_info: Vec<NutritionInfo>
}

#[derive(Deserialize, Debug)]
pub struct NutritionInfo {
    pub label: NutritionInfoLabel,
    pub value: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum NutritionInfoLabel {
    #[serde(rename = "Energie")]
    Energy,
    #[serde(rename = "Fett")]
    Fat,
    #[serde(rename = "Fett, davon gesättigte Fettsäuren")]
    FatSaturatedFattyAcids,
    #[serde(rename = "Kohlenhydrate")]
    Carbohydrates,
    #[serde(rename = "Kohlenhydrate, davon Zucker")]
    CarbohydratesSugar,
    #[serde(rename = "Eiweiß")]
    Protein,
    #[serde(rename = "Salz")]
    Salt
}

#[derive(Deserialize, Debug)]
pub struct GeneralInformationItem {
    pub title: String,
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct ContactInformationItem {
    pub title: ContactInformationTitle,
    pub description: String
}

#[derive(Deserialize, Debug)]
pub enum ContactInformationTitle {
    #[serde(rename = "Herstellername")]
    ManufacturerName,
    #[serde(rename = "Herstelleradresse")]
    ManufacturerAddress,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecommendProduct {
    pub name: String,
    pub grammage: String,
    pub current_price: String,
    pub image_url: String,
    pub tags: Vec<String>,
    pub listing_id: String,
    pub id: String
}