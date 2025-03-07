use super::super::processor::Processor;
use super::{Pipeline, PipelineData};
use crate::data::Context;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};
use std::sync::Arc;

pub struct SyncPipeline<ProcessContextOut: PipelineData> {
    processor: Arc<dyn Processor<Output = ProcessContextOut>>,
}

impl<ProcessContextOut: PipelineData> SyncPipeline<ProcessContextOut> {
    pub fn new(processor: impl Processor<Output = ProcessContextOut> + 'static) -> Self {
        Self {
            processor: Arc::new(processor),
        }
    }
}

impl<ProcessContextOut: PipelineData> Pipeline for SyncPipeline<ProcessContextOut> {
    type ProcessContextOut = ProcessContextOut;

    fn conduct(
        &self,
        tx: &Sender<Self::ProcessContextOut>,
        rx: &Receiver<Context>,
    ) -> Result<(), Box<dyn Error + Send>> {
        self.processor.process(tx, rx)
    }
}
