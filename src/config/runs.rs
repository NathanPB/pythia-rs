use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static ERRCODE_RUN_NAME_DUPE: &str = "ERRCODE_RUN_NAME_DUPE";

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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RunValue {
    Bool(bool),
    Number(serde_json::Number),
    String(String),
}

#[derive(Validate, Serialize, Deserialize, Clone, Debug)]
pub struct RunConfig {
    #[validate(regex(path = *RE_VALID_RUN_NAME, message = "Run name must be alphanumeric and contain only underscores and dashes"))]
    pub name: String,

    #[serde(flatten)]
    pub extra: HashMap<String, RunValue>,
}
