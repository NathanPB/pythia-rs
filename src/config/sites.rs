use crate::config::Config;
use crate::registry::resources::SiteGeneratorDriverResource;
use crate::registry::{PublicIdentifier, PublicIdentifierSeed, Registry};
use crate::sites::SiteGenerator;
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde_json::Map;
use std::error::Error;
use std::fmt;
use validator::Validate;

#[derive(Validate, Clone, Debug)]
pub struct SiteSourceConfig {
    pub source_type: PublicIdentifier,
    pub sample_size: Option<usize>,
    args: serde_json::Value,
}

impl Config {
    pub fn sites(
        &self,
        registry: &Registry<SiteGeneratorDriverResource>,
    ) -> Result<impl SiteGenerator, Box<dyn Error>> {
        let id = &self.sites.source_type;
        let driver = registry.get(id)
            .ok_or(
                format!(
                    "Sites source driver under the ID {} couldn't be found. Are you sure that this resource exists (or their plugin is loaded?).",
                    id
                ).as_str()
            )?;

        let config = (driver.0.config_deserializer)(self.sites.args.clone())?;
        (driver.0.create)(config)
    }
}

#[derive(Clone)]
pub struct SiteSourceConfigSeed {
    pub id_seed: PublicIdentifierSeed,
}

impl<'de> DeserializeSeed<'de> for SiteSourceConfigSeed {
    type Value = SiteSourceConfig;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(SiteSourceConfigVisitor { seed: self })
    }
}

struct SiteSourceConfigVisitor {
    seed: SiteSourceConfigSeed,
}

impl<'de> Visitor<'de> for SiteSourceConfigVisitor {
    type Value = SiteSourceConfig;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a SiteSourceConfig struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut source_type = None;
        let mut sample_size = None;
        let mut args: Map<String, serde_json::Value> = Map::new();

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => source_type = Some(map.next_value_seed(self.seed.id_seed.clone())?),
                "sample_size" => sample_size = Some(map.next_value()?),
                _ => {
                    args.insert(key.to_string(), map.next_value()?);
                }
            }
        }

        let source_type = source_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
        Ok(SiteSourceConfig {
            source_type,
            sample_size,
            args: serde_json::Value::Object(args),
        })
    }
}
