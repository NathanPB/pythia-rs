pub mod runs;
pub mod sites;

use crate::config::sites::SiteSourceConfig;
use clap::Parser;
use runs::*;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use std::borrow::Cow;
use std::error::Error;
use std::path::PathBuf;
use validator::{Validate, ValidationError};

static ERRCODE_WORKDIR_NOT_DIR: &str = "ERRCODE_WORKDIR_NOT_DIR";
static ERRCODE_WORKDIR_NOT_EMPTY: &str = "ERRCODE_WORKDIR_NOT_EMPTY";

fn validate_workdir_is_directory(path: &PathBuf) -> Result<(), ValidationError> {
    if path.exists() && !path.is_dir() {
        return Err(
            ValidationError::new(ERRCODE_WORKDIR_NOT_DIR).with_message(Cow::from(format!(
                "Working directory {} is not a directory.",
                path.display()
            ))),
        );
    }

    Ok(())
}

fn validate_workdir_overrides(args: &Args) -> Result<(), ValidationError> {
    if let Some(path) = &args.workdir {
        if !args.clear_workdir {
            match path.read_dir() {
                Ok(entries) => {
                    if entries.count() > 0 {
                        let msg = format!("Working directory {} is not empty. Specify --clear-workdir to FORCEFULLY OVERWRITE it.", path.display());
                        return Err(ValidationError::new(ERRCODE_WORKDIR_NOT_EMPTY)
                            .with_message(Cow::from(msg)));
                    }
                }
                Err(err) => match err.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => panic!(
                        "Unexpected error when checking workdir availability: {}",
                        err
                    ),
                },
            }
        }
    }
    Ok(())
}

#[derive(Validate, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the JSON configuration file.
    #[arg(short, long, default_value = "config.json")]
    pub config_file: String,

    /// Number of workers to use for parallel processing. If 0, will use all available cores.
    #[arg(short, long, default_value_t = 0)]
    pub workers: usize,

    /// Size of the buffer between each step of the processing pipeline. Defaults to 128.
    #[arg(short, long, default_value_t = 128)]
    pub pipeline_buffer_size: usize,

    /// Specify the working directory, created recursively if needed. If not specified, a temporary one will be created.
    /// Check --keep-workdir and --clear-workdir to control the behavior of the working directory.
    /// By default, the program will halt execution if the specified --workdir is not empty, unless --clear-workdir is specified.
    #[arg(short = 'd', long)]
    #[validate(custom(function = "validate_workdir_is_directory"))]
    pub workdir: Option<PathBuf>,

    /// Keeps the working directory after completed. Defaults to true if --workdir is specified.
    /// This option has NO effect if combined with --workdir (directory will always be kept).
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub keep_workdir: Option<bool>,

    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    /// Overrides the working directory if it isn't already empty. This option has NO effect if not combined with --workdir (directory will always be kep).
    /// By default, the program will halt execution if the specified --workdir is not empty, unless --clear-workdir is specified.
    pub clear_workdir: bool,
}

#[derive(Debug)]
pub struct ConfigError(pub Box<dyn Error>);
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.0)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to load config file: {}", self.0)
    }
}

#[derive(Debug)]
pub struct ArgsError(pub Box<dyn Error>);
impl Error for ArgsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.0)
    }
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to validate arguments: {}", self.0)
    }
}

#[serde_inline_default]
#[derive(Validate, Deserialize, Clone, Debug)]
pub struct Config {
    pub sites: SiteSourceConfig,

    #[validate(length(min = 1, message = "At least one run is required"))]
    #[validate(nested)]
    #[validate(custom(function = "validate_unique_run_names"))]
    pub runs: Vec<RunConfig>,
}

fn validate(args: &Args, config: &Config) -> Result<(), Box<dyn Error>> {
    args.validate().map_err(|e| ArgsError(Box::new(e)))?;
    validate_workdir_overrides(args).map_err(|e| ArgsError(Box::new(e)))?;
    config.validate().map_err(|e| ConfigError(Box::new(e)))?;

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
