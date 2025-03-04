pub mod config;
pub mod drivers;
pub mod gen;

use crate::data::Site;
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

/// Constructs a new [`SiteGenerator`] of type [`G`] from the config [`C`].
#[allow(type_alias_bounds)] // I prefer to keep the constraint here for when this makes its way into stable Rust.
type SitegenFactory<G: SiteGenerator, C> = Arc<Box<dyn Fn(C) -> Result<G, Box<dyn Error>>>>;

/// Deserializes a config of type [`C`] from a [`serde_json::Value`].
type SitegenConfigDeserializer<C> =
    Arc<Box<dyn Fn(serde_json::Value) -> Result<C, serde_json::error::Error>>>;

/// SiteGenerator allows for streaming Sites from an undetermined source.
/// The order of the sites is not guaranteed, as different file formats may index their data differently, and pre-sorting is not possible.
pub trait SiteGenerator: Iterator<Item = Site> {}
impl<T: Iterator<Item = Site>> SiteGenerator for T {}

pub struct SiteGeneratorDriver<G: SiteGenerator, C> {
    pub create: SitegenFactory<G, C>,
    pub config_deserializer: SitegenConfigDeserializer<C>,
}

impl<G: SiteGenerator, C> Clone for SiteGeneratorDriver<G, C> {
    fn clone(&self) -> Self {
        SiteGeneratorDriver {
            create: self.create.clone(),
            config_deserializer: self.config_deserializer.clone(),
        }
    }
}

impl<G: SiteGenerator, C> SiteGeneratorDriver<G, C> {
    pub fn coerce_to_dynamic(self) -> SiteGeneratorDriver<Box<dyn SiteGenerator>, Box<dyn Any>>
    where
        G: SiteGenerator + 'static,
        C: Any + 'static,
    {
        SiteGeneratorDriver {
            create: Arc::new(Box::new(move |c: Box<dyn Any>| {
                let config = c
                    .downcast::<C>()
                    .map_err(|_| Box::<dyn Error>::from("Failed to downcast config"))?;
                let concrete_generator = (self.create)(*config)?;
                Ok(Box::new(concrete_generator) as Box<dyn SiteGenerator>)
            })),
            config_deserializer: Arc::new(Box::new(move |v| {
                let concrete_config = (self.config_deserializer)(v)?;
                Ok(Box::new(concrete_config) as Box<dyn Any>)
            })),
        }
    }
}
