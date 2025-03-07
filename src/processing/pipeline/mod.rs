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
    type Output: PipelineData;
    fn conduct(
        &self,
        tx: &Sender<Self::Output>,
        rx: &Receiver<Context>,
    ) -> Result<(), Box<dyn Error + Send>>;
}

pub fn create_pipeline_from_config<'a, O: PipelineData + 'static>(
    _config: &Config,
    workers: usize,
    processor: impl Processor<Output = O> + 'static,
) -> Result<Pipelines<O>, Box<dyn Error>> {
    let worker_count = match workers {
        0 => num_cpus::get(),
        workers => workers,
    };

    let pipeline: Pipelines<O> = match workers {
        1 => Pipelines::SYNC(SyncPipeline::new(processor)),
        _ => Pipelines::THREADED(ThreadedPipeline::new(processor, worker_count)?),
    };

    Ok(pipeline)
}

pub enum Pipelines<T: PipelineData> {
    SYNC(SyncPipeline<T>),
    THREADED(ThreadedPipeline<T>),
}
