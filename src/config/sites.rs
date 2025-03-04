use crate::config::Config;
use crate::registry::resources::SiteGeneratorDriverResource;
use crate::registry::{Namespace, Registry, RE_VALID_NAMESPACE_AND_ID};
use crate::sites::SiteGenerator;
use serde::Deserialize;
use std::error::Error;
use validator::Validate;

#[derive(Validate, Deserialize, Clone, Debug)]
pub struct SiteSourceConfig {
    #[validate(regex(
        path = "*RE_VALID_NAMESPACE_AND_ID",
        message = "Site source must be in the format of `<namespace>:<id>`. Examples are `foo:bar` or `bar` (assumed to be in the default namespace)."
    ))]
    #[validate(length(min = 1, message = "Site source type cannot be empty."))]
    #[serde(rename = "type")]
    pub source_type: String,

    #[serde(flatten)]
    args: serde_json::Value,
}

impl Config {
    #[allow(dead_code)] // The functionality required by this haven't made its way into the entrypoint yet, but this fn definitely isn't dead code.
    pub fn sites(
        &self,
        default_namespace: &Namespace,
        registry: &Registry<SiteGeneratorDriverResource>,
    ) -> Result<impl SiteGenerator, Box<dyn Error>> {
        let captures = RE_VALID_NAMESPACE_AND_ID
            .captures(&self.sites.source_type)
            .unwrap(); // regex validation has already been performed, so we are sure that this unwraps. If it doesn't, it's a good reason to panic.

        let namespace = captures
            .name("ns")
            .map(|m| m.as_str())
            .unwrap_or(default_namespace.namespace());

        let id = captures.name("id").map(|m| m.as_str()).unwrap(); // I'm sure that "id" exists. If it doesn't, it's a good reason to panic.

        let driver = registry.get_foreign(namespace, id)
            .ok_or(
                format!(
                    "Sites source driver under the ID {}:{} couldn't be found. Are you sure that this resource exists (or their plugin is loaded?).",
                    namespace, id
                ).as_str()
            )?;

        println!("Using site source driver: {}", self.sites.args.clone());
        let config = (driver.0.config_deserializer)(self.sites.args.clone())?;
        println!("Sites config: {:?}", config);

        (driver.0.create)(config)
    }
}
