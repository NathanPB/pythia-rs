use super::super::processor::Processor;
use super::{Pipeline, PipelineData};
use crate::data::Context;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::ScopedJoinHandle;

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

pub struct ThreadedPipelineBuilder<ProcessContextOut: PipelineData> {
    pub workers: usize,
    pub processor: Arc<dyn Processor<Output = ProcessContextOut>>,
}

impl<ProcessContextOut: PipelineData> ThreadedPipeline<ProcessContextOut> {
    pub fn builder(
        processor: impl Processor<Output = ProcessContextOut> + 'static,
        workers: usize,
    ) -> ThreadedPipelineBuilder<ProcessContextOut> {
        ThreadedPipelineBuilder {
            workers,
            processor: Arc::new(processor),
        }
    }
}

impl<ProcessContextOut: PipelineData> ThreadedPipelineBuilder<ProcessContextOut> {
    pub fn build(self) -> Result<ThreadedPipeline<ProcessContextOut>, Box<dyn Error>> {
        if self.workers <= 1 {
            return Err(NotEnoughWorkersError.into());
        }

        Ok(ThreadedPipeline {
            workers: self.workers,
            processor: self.processor,
        })
    }
}

pub struct ThreadedPipeline<ProcessContextOut: Sized + Send + Sync> {
    workers: usize,
    processor: Arc<dyn Processor<Output = ProcessContextOut>>,
}

impl<ProcessContextOut: PipelineData + 'static> Pipeline for ThreadedPipeline<ProcessContextOut> {
    type ProcessContextOut = ProcessContextOut;

    fn conduct(
        &self,
        tx: &Sender<Self::ProcessContextOut>,
        rx: &Receiver<Context>,
    ) -> Result<(), Box<dyn Error + Send>> {
        thread::scope(|s| {
            let thread_pool: Vec<ScopedJoinHandle<()>> = (0..self.workers)
                .enumerate()
                .map(move |(i, _)| {
                    s.spawn(move || {
                        // TODO better error recovery
                        if let Err(err) = self.processor.process(&tx, &rx) {
                            panic!("ThreadedPipeline: Worker Thread {} crashed: {}", i, err);
                        }
                    })
                })
                .collect();

            thread_pool.into_iter().enumerate().for_each(|(i, t)| {
                t.join().expect(&format!(
                    "ThreadedPipeline: Worker Thread {} crashed when joining.",
                    i
                ));
            });

            Ok(())
        })
    }
}
