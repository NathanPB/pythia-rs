pub mod config;
pub mod drivers;

use crate::io::sitegen::*;
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

pub struct SiteGeneratorDriver<G: SiteGenerator, C> {
    pub create: Arc<Box<dyn Fn(C) -> Result<G, Box<dyn Error>>>>,
    pub config_deserializer:
        Arc<Box<dyn Fn(serde_json::Value) -> Result<C, serde_json::error::Error>>>,
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
