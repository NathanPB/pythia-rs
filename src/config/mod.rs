pub mod runs;
pub mod sites;

use crate::config::sites::SiteSourceConfig;
use clap::Parser;
use runs::*;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use std::error::Error;
use std::path::PathBuf;
use validator::Validate;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the JSON configuration file.
    #[arg(short, long, default_value = "config.json")]
    pub config_file: String,

    /// Number of workers to use for parallel processing. If 0, will use all available cores.
    #[arg(short, long, default_value_t = 0)]
    pub workers: u16,
}

#[serde_inline_default]
#[derive(Validate, Deserialize, Clone, Debug)]
pub struct Config {
    sites: SiteSourceConfig,

    #[validate(length(min = 1, message = "At least one run is required"))]
    #[validate(nested)]
    #[validate(custom(function = "validate_unique_run_names"))]
    pub runs: Vec<RunConfig>,
}

fn validate(_: &Args, config: &Config) -> Result<(), Box<dyn Error>> {
    config.validate()?;

    Ok(())
}

#[derive(Debug)]
pub struct ConfigFileNotFoundError(PathBuf);

impl Error for ConfigFileNotFoundError {}

impl std::fmt::Display for ConfigFileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config file not found at path {}", self.0.display())
    }
}

pub fn init() -> Result<(Config, Args, PathBuf), Box<dyn Error>> {
    let args = Args::parse();
    let path = PathBuf::from(&args.config_file.clone());
    if !path.exists() || !path.is_file() {
        return Err(Box::new(ConfigFileNotFoundError(path.clone())));
    }

    let json_str = std::fs::read_to_string(args.config_file.clone())?;
    let config: Config = serde_json::from_str(&json_str)?;

    validate(&args, &config)?;

    Ok((config, args, path))
}
