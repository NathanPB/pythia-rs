use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static ERRCODE_RUN_NAME_DUPE: &str = "ERRCODE_RUN_NAME_DUPE";

static RE_VALID_RUN_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

fn validate_unique_run_names(runs: &Vec<RunConfig>) -> Result<(), ValidationError> {
    let mut run_names = HashSet::new();

    for run in runs {
        if run_names.contains(&run.name) {
            let msg = format!("Run name {} is not unique", run.name);
            return Err(ValidationError::new(ERRCODE_RUN_NAME_DUPE).with_message(Cow::from(msg)));
        }
        run_names.insert(run.name.clone());
    }

    Ok(())
}

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

#[derive(Validate, Deserialize, Clone, Debug)]
pub struct Config {
    pub sites: SitesSource,

    #[validate(length(min = 1, message = "At least one run is required"))]
    #[validate(nested)]
    #[validate(custom(function = "validate_unique_run_names"))]
    pub runs: Vec<RunConfig>,
}

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

#[derive(Validate, Serialize, Deserialize, Clone, Debug)]
pub struct RunConfig {
    #[validate(regex(path = *RE_VALID_RUN_NAME, message = "Run name must be alphanumeric and contain only underscores and dashes"))]
    pub name: String,
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

pub fn init<'a>() -> Result<(Config, Args, PathBuf), Box<dyn Error>> {
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
