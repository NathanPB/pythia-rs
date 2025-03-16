use super::resources::*;
use super::{Namespace, Registry};
use crate::sites::drivers::*;
use std::error::Error;

pub fn init_itself(registries: &mut super::Registries) -> Result<Namespace, Box<dyn Error>> {
    let namespace = registries.claim_namespace("std")?;
    register_sitegen_drivers(&namespace, registries.regmut_sitegen_drivers())?;
    Ok(namespace)
}

fn register_sitegen_drivers(
    namespace: &Namespace,
    registry: &mut Registry<SiteGeneratorDriverResource>,
) -> Result<(), Box<dyn Error>> {
    registry.register(
        &namespace,
        "vector",
        SiteGeneratorDriverResource(DRIVER_VECTOR.clone().coerce_to_dynamic()),
    )?;

    registry.register(
        &namespace,
        "raster",
        SiteGeneratorDriverResource(DRIVER_RASTER.clone().coerce_to_dynamic()),
    )?;

    Ok(())
}
