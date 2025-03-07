pub mod unbatched;

use super::PipelineData;
use crate::data::Context;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};

pub trait Processor: Send + Sync {
    type Output: PipelineData;

    fn process(
        &self,
        tx: &Sender<Self::Output>,
        rx: &Receiver<Context>,
    ) -> Result<(), Box<dyn Error + Send>>;
}
