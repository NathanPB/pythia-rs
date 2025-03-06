mod sync;
mod threaded;

use super::pipeline::threaded::ThreadedPipeline;
use crate::config::Config;
use crate::data::Context;
use std::error::Error;
pub use sync::*;

pub trait Pipeline {
    fn submit(&self, context: Context) -> Result<(), Box<dyn Error>>;
}

pub fn create_pipeline_from_config(
    _config: &Config,
    workers: usize,
) -> Result<Box<dyn Pipeline>, Box<dyn Error>> {
    let worker_count = match workers {
        0 => num_cpus::get(),
        workers => workers,
    };

    let processor: Box<dyn Fn(Context) -> Result<(), Box<dyn Error>> + Send + Sync> =
        Box::new(|context: Context| {
            println!("Fake-processing context {:?}", context);
            Ok(())
        });

    let pipeline: Box<dyn Pipeline> = match workers {
        1 => Box::new(SyncPipeline { processor }),
        _ => Box::new(ThreadedPipeline::builder(processor, worker_count).build()?),
    };

    Ok(pipeline)
}
