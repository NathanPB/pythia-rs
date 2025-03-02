use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use validator::Validate;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SitesSource {
    Vector(VectorSitesSourceConfig),
    Raster(RasterSitesSourceConfig),
}

#[serde_inline_default]
#[derive(Validate, Deserialize, Clone, Debug)]
pub struct VectorSitesSourceConfig {
    #[validate(length(min = 1, message = "Vector file path cannot be empty"))]
    pub file: String,

    #[serde_inline_default("ID".to_string())]
    #[validate(length(min = 1, message = "Site ID key cannot be empty"))]
    pub site_id_key: String,
}

#[serde_inline_default]
#[derive(Validate, Deserialize, Clone, Debug)]
pub struct RasterSitesSourceConfig {
    pub file: String,

    #[serde_inline_default(0)]
    pub layer_index: usize,
}
