use super::super::context::Context;
use super::Processor;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::mpmc::{Receiver, Sender};

pub struct UnbatchedProcessor {
    pub workdir: PathBuf,
}

impl Processor for UnbatchedProcessor {
    type Output = Context;

    fn process(
        &self,
        tx: &Sender<Self::Output>,
        rx: &Receiver<Self::Output>,
    ) -> Result<(), Box<dyn Error + Send>> {
        rx.iter()
            .map(|ctx| {
                let path = ctx.dir(&self.workdir);
                if let Err(err) = create_dir_all(&path) {
                    eprintln!("UnbatchedProcessor: Failed to create directory: {}", err);
                }

                ctx
            })
            .try_for_each(|ctx| tx.send(ctx))
            .map_err(|err| Box::new(err) as Box<dyn Error + Send>)
    }
}
