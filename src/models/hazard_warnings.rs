use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HazardAndWarnings {
    pub age_verification_key: Option<String>,
    pub warning_signs: Vec<WarningSign>
}

#[derive(Deserialize, Debug)]
pub enum WarningSign {
    #[serde(rename = "Biozid")]
    Biocide
}