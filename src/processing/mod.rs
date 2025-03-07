use crate::config::{Args, Config};
use crate::data::Context;
use crate::registry::{Namespace, Registries};
use context::ContextGenerator;
use pipeline::{create_pipeline_from_config, Pipeline, Pipelines};
use processor::unbatched::UnbatchedProcessor;
use std::sync::mpmc::sync_channel;
use std::sync::Arc;
use std::thread;

mod context;
mod pipeline;
mod processor;

pub trait PipelineData: Sized + Send + Sync {}

pub struct ProcessingBuilder<'a> {
    pub config: &'a Config,
    pub args: &'a Args,
    pub default_namespace: &'a Namespace,
    pub registries: &'a mut Registries,
}

impl<'a> ProcessingBuilder<'a> {
    pub fn build(self) -> Result<Processing<Context>, Box<dyn std::error::Error>> {
        let sitegen = self.config.sites(
            &self.default_namespace,
            &self.registries.reg_sitegen_drivers(),
        )?;

        let ctx_gen = ContextGenerator::new(Box::new(sitegen), self.config.runs.clone())?;

        let processor = UnbatchedProcessor {};

        let pipeline = create_pipeline_from_config(self.config, self.args.workers, processor)?;

        Ok(Processing {
            pipeline,
            ctx_gen,
            buffer_size: self.args.pipeline_buffer_size,
        })
    }
}

pub struct Processing<T: PipelineData> {
    pipeline: Pipelines<T>,
    ctx_gen: ContextGenerator,
    buffer_size: usize,
}

impl<T: PipelineData + 'static> Processing<T> {
    pub fn start(self) {
        let ctx_gen = self.ctx_gen;
        let pipeline: Arc<dyn Pipeline<ProcessContextOut = T>> = match self.pipeline {
            Pipelines::SYNC(pipeline) => Arc::new(pipeline),
            Pipelines::THREADED(pipeline) => Arc::new(pipeline),
        };

        thread::scope(|s| {
            let (tx, rx_conduct) = sync_channel::<Context>(self.buffer_size);
            let (tx_conduct, rx) = sync_channel::<T>(self.buffer_size);

            let tx_conduct2 = tx_conduct.clone();
            let t_conductor = s.spawn(move || pipeline.conduct(&tx_conduct2, &rx_conduct).unwrap());
            let t_sink = s.spawn(move || {
                for _ in rx { /* noop */ }
            });

            for ctx in ctx_gen {
                tx.send(ctx).unwrap();
            }

            drop(tx);
            t_conductor.join().unwrap();

            drop(tx_conduct);
            t_sink.join().unwrap();
        })
    }
}
