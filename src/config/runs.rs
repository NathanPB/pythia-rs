use crate::processing::context::ContextValue;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static ERRCODE_RUN_NAME_DUPE: &str = "ERRCODE_RUN_NAME_DUPE";
static ERRCODE_TEMPLATE_FILE_NOT_FOUND: &str = "ERRCODE_TEMPLATE_FILE_NOT_FOUND";

static RE_VALID_RUN_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

pub fn validate_unique_run_names(runs: &Vec<RunConfig>) -> Result<(), ValidationError> {
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

fn validate_template_file_exists(path: &PathBuf) -> Result<(), ValidationError> {
    if !path.exists() || path.is_dir() {
        let msg = format!(
            "Template file {} does not exist or is not a file",
            path.display()
        );

        return Err(
            ValidationError::new(ERRCODE_TEMPLATE_FILE_NOT_FOUND).with_message(Cow::from(msg))
        );
    }
    Ok(())
}

#[derive(Validate, Serialize, Deserialize, Clone, Debug)]
pub struct RunConfig {
    #[validate(regex(path = *RE_VALID_RUN_NAME, message = "Run name must be alphanumeric and contain only underscores and dashes"))]
    pub name: String,

    #[validate(custom(function = "validate_template_file_exists"))]
    pub template: PathBuf,

    #[serde(flatten)]
    pub extra: HashMap<String, ContextValue>,
}
