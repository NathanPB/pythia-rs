use super::*;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct AlreadyRegisteredError(pub PublicIdentifier);

impl Error for AlreadyRegisteredError {}

impl Display for AlreadyRegisteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier {} is already registered.", self.0.id)
    }
}

#[derive(Debug)]
pub struct NamespaceAlreadyClaimedError(
    #[allow(dead_code)] // This is part of the public API, so it's not dead code.
    pub  Namespace,
);

impl Error for NamespaceAlreadyClaimedError {}

impl Display for NamespaceAlreadyClaimedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace is already claimed.")
    }
}

#[derive(Debug)]
pub struct IllegalNameError(
    #[allow(dead_code)] // This is part of the public API, so it's not dead code.
    pub  String,
);

impl Error for IllegalNameError {}

impl Display for IllegalNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The provided name is empty or contains illegal characters. Only lowercase alphanumeric and dash characters are allowed.")
    }
}
