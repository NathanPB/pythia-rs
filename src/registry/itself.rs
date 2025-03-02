use std::error::Error;

pub fn init_itself(registries: &mut super::Registries) -> Result<super::Namespace, Box<dyn Error>> {
    let namespace = registries.claim_namespace("std").unwrap();

    // TODO register own resources here

    Ok(namespace)
}
