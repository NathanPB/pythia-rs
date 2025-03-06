use crate::config::{Args, Config};
use crate::processing::context::ContextGenerator;
use crate::processing::pipeline::{create_pipeline_from_config, Pipeline};
use crate::registry::{Namespace, Registries};

mod context;
mod pipeline;

pub struct ProcessingBuilder<'a> {
    pub config: &'a Config,
    pub args: &'a Args,
    pub default_namespace: &'a Namespace,
    pub registries: &'a mut Registries,
}

impl<'a> ProcessingBuilder<'a> {
    pub fn build(self) -> Result<Processing, Box<dyn std::error::Error>> {
        let sitegen = self.config.sites(
            &self.default_namespace,
            &self.registries.reg_sitegen_drivers(),
        )?;

        let ctx_gen = ContextGenerator::new(Box::new(sitegen), self.config.runs.clone())?;

        let pipeline = create_pipeline_from_config(self.config, self.args.workers)?;

        Ok(Processing { pipeline, ctx_gen })
    }
}

pub struct Processing {
    pipeline: Box<dyn Pipeline>,
    ctx_gen: ContextGenerator,
}

impl Processing {
    pub fn start(self) {
        let ctx_gen = self.ctx_gen;
        let pipeline = self.pipeline;

        for ctx in ctx_gen {
            pipeline.submit(ctx).unwrap();
        }
    }
}
