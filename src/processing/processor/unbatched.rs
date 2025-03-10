use super::super::context::Context;
use super::super::template::TemplateEngine;
use super::Processor;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::mpmc::{Receiver, Sender};

pub struct UnbatchedProcessor {
    pub workdir: PathBuf,
}

impl Processor for UnbatchedProcessor {
    type Output = Context;

    fn process(
        &self,
        tx: &Sender<Self::Output>,
        rx: &Receiver<Self::Output>,
        templates: &TemplateEngine,
    ) -> Result<(), Box<dyn Error + Send>> {
        // TODO better error handling
        rx.iter()
            .map(|ctx| {
                let path = ctx.dir(&self.workdir);
                if let Err(err) = create_dir_all(&path) {
                    eprintln!("UnbatchedProcessor: Failed to create directory: {}", err);
                }
                ctx
            })
            .map(|ctx| {
                let filename = match templates.file_name(ctx.run.name.as_str()) {
                    Some(filename) => filename,
                    None => {
                        panic!(
                            "Failed to render template for context ID {} ({}, {}): Template file name not registered",
                            ctx.site.id, ctx.site.lon, ctx.site.lat
                        );
                    }
                };

                let rendered = templates.render(&ctx).unwrap();
                let mut template_path = ctx.dir(&self.workdir);
                template_path.push(filename);

                if let Err(err) = std::fs::write(template_path, rendered) {
                    panic!(
                        "Failed to render template for context ID {} ({}, {}): {}",
                        ctx.site.id, ctx.site.lon, ctx.site.lat, err
                    );
                }

                ctx
            })
            .try_for_each(|ctx| tx.send(ctx))
            .map_err(|err| Box::new(err) as Box<dyn Error + Send>)
    }
}
