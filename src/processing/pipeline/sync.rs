use crate::data::Context;
use crate::processing::pipeline::Pipeline;

pub struct SyncPipeline {
    pub processor: Box<dyn Fn(Context) -> Result<(), Box<dyn std::error::Error>> + Send + Sync>,
}

impl Pipeline for SyncPipeline {
    fn submit(&self, context: Context) -> Result<(), Box<dyn std::error::Error>> {
        (self.processor)(context)
    }
}
