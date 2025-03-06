use crate::data::Context;
use crate::processing::pipeline::Pipeline;
use std::error::Error;
use std::sync::Arc;
use threadpool::ThreadPool;

#[derive(Debug)]
struct NotEnoughWorkersError;
impl Error for NotEnoughWorkersError {}

impl std::fmt::Display for NotEnoughWorkersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Not enough workers to use ThreadedPipelineBuilder. At least 2 workers are required."
        )
    }
}

pub struct ThreadedPipelineBuilder {
    pub workers: usize,
    pub processor: Box<dyn Fn(Context) -> Result<(), Box<dyn Error>> + Send + Sync>,
}

impl ThreadedPipeline {
    pub fn builder(
        processor: Box<dyn Fn(Context) -> Result<(), Box<dyn Error>> + Send + Sync>,
        workers: usize,
    ) -> ThreadedPipelineBuilder {
        ThreadedPipelineBuilder { workers, processor }
    }
}

impl ThreadedPipelineBuilder {
    pub fn build(self) -> Result<ThreadedPipeline, Box<dyn Error>> {
        if self.workers <= 1 {
            return Err(NotEnoughWorkersError.into());
        }

        Ok(ThreadedPipeline {
            pool: ThreadPool::new(self.workers),
            processor: Arc::new(self.processor),
        })
    }
}

pub struct ThreadedPipeline {
    pool: ThreadPool,
    processor: Arc<dyn Fn(Context) -> Result<(), Box<dyn Error>> + Send + Sync>,
}

impl Pipeline for ThreadedPipeline {
    fn submit(&self, context: Context) -> Result<(), Box<dyn Error>> {
        let processor = self.processor.clone();
        self.pool.execute(move || {
            processor(context).expect("Error in threaded pipeline");
        });

        Ok(())
    }
}

impl Drop for ThreadedPipeline {
    fn drop(&mut self) {
        self.pool.join();
    }
}
