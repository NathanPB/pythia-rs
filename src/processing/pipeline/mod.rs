mod sync;
mod threaded;

use super::processor::Processor;
use super::PipelineData;
use crate::config::Config;
use crate::data::Context;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};
pub use sync::*;
pub use threaded::*;

pub trait Pipeline: Send + Sync {
    type ProcessContextOut: PipelineData;
    fn conduct(
        &self,
        tx: &Sender<Self::ProcessContextOut>,
        rx: &Receiver<Context>,
    ) -> Result<(), Box<dyn Error + Send>>;
}

pub fn create_pipeline_from_config<'a, ProcessorContextOut: PipelineData + 'static>(
    _config: &Config,
    workers: usize,
    processor: impl Processor<Output = ProcessorContextOut> + 'static,
) -> Result<Pipelines<ProcessorContextOut>, Box<dyn Error>> {
    let worker_count = match workers {
        0 => num_cpus::get(),
        workers => workers,
    };

    let pipeline: Pipelines<ProcessorContextOut> = match workers {
        1 => Pipelines::SYNC(SyncPipeline::new(processor)),
        _ => Pipelines::THREADED(ThreadedPipeline::builder(processor, worker_count).build()?),
    };

    Ok(pipeline)
}

pub enum Pipelines<T: PipelineData> {
    SYNC(SyncPipeline<T>),
    THREADED(ThreadedPipeline<T>),
}
