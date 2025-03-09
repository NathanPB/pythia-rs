use super::context::Context;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub struct TemplateEngine {
    tera: tera::Tera,
    filenames: HashMap<String, String>,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        TemplateEngine {
            tera: tera::Tera::default(),
            filenames: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct TemplateError(String);

impl Error for TemplateError {}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TemplateEngine {
    pub fn register(&mut self, run_name: &str, file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let full_path = file.canonicalize()?;
        let contents = std::fs::read_to_string(full_path)?;

        self.tera.add_raw_template(run_name, contents.as_str())?;
        self.filenames.insert(run_name.to_string(), file.file_name().expect("TemplateEngine: Can't register because the template file doesn't look like a file.").to_string_lossy().to_string());
        Ok(())
    }

    pub fn file_name(&self, run_name: &str) -> Option<&String> {
        self.filenames.get(run_name)
    }

    /// TODO: error handling
    pub fn render(&self, ctx: &Context) -> String {
        self.tera
            .render(ctx.run.name.as_str(), &ctx.tera())
            .unwrap()
    }
}
