use super::config::*;
use super::SiteGeneratorDriver;
use crate::sites::gen::*; // TODO move sitegen to sites::gen
use std::sync::{Arc, LazyLock};

pub const DRIVER_VECTOR: LazyLock<
    SiteGeneratorDriver<VectorSiteGenerator, VectorSiteGeneratorConfig>,
> = LazyLock::new(|| SiteGeneratorDriver {
    create: Arc::new(|c: VectorSiteGeneratorConfig| {
        VectorSiteGenerator::new(c.file.as_str(), c.site_id_key)
    }),
    config_deserializer: Arc::new(serde_json::from_value),
});

pub const DRIVER_RASTER: LazyLock<
    SiteGeneratorDriver<RasterSiteGenerator, RasterSiteGeneratorConfig>,
> = LazyLock::new(|| SiteGeneratorDriver {
    create: Arc::new(|c: RasterSiteGeneratorConfig| {
        RasterSiteGenerator::new(c.file.as_str(), c.layer_index)
    }),
    config_deserializer: Arc::new(serde_json::from_value),
});
