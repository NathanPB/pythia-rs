use super::super::context::Context;
use super::Processor;
use std::error::Error;
use std::sync::mpmc::{Receiver, Sender};

pub struct UnbatchedProcessor {}

fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
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
                fib(30); // TODO replace with the actual workflow
                ctx
            })
            .try_for_each(|ctx| tx.send(ctx))
            .map_err(|err| Box::new(err) as Box<dyn Error + Send>)
    }
}
