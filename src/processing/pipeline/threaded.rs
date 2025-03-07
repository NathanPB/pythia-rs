use super::super::processor::Processor;
use super::{Pipeline, PipelineData};
use crate::data::Context;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::ScopedJoinHandle;

#[derive(Debug)]
pub struct NotEnoughWorkersError;
impl Error for NotEnoughWorkersError {}

impl std::fmt::Display for NotEnoughWorkersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Not enough workers to use ThreadedPipelineBuilder. At least 2 workers are required."
        )
    }
}

impl<ProcessContextOut: PipelineData> ThreadedPipeline<ProcessContextOut> {
    pub fn new(
        processor: impl Processor<Output = ProcessContextOut> + 'static,
        workers: usize,
    ) -> Result<ThreadedPipeline<ProcessContextOut>, NotEnoughWorkersError> {
        if workers <= 1 {
            return Err(NotEnoughWorkersError.into());
        }

        Ok(ThreadedPipeline {
            workers,
            processor: Arc::new(processor),
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
