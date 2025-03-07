#![feature(mpmc_channel)]

mod config;
mod data;
mod processing;
mod registry;
mod sites;
mod utils;

use crate::processing::ProcessingBuilder;
use registry::{itself::init_itself, Registries};

fn main() {
    let cfg_result = config::init();
    if let Err(e) = cfg_result {
        println!("Unable to load config file: {}", e);
        return;
    }

    let mut registries = Registries::new();
    let namespace = init_itself(&mut registries).unwrap();
    println!("Initialized own resources on namespace \"{}\"", namespace);

    let (config, _args, config_file) = cfg_result.unwrap();
    println!(
        "Loaded configuration file from {}",
        config_file.canonicalize().ok().unwrap().display()
    );

    let processing = ProcessingBuilder {
        config: &config,
        args: &_args,
        default_namespace: &namespace,
        registries: &mut registries,
    }
    .build()
    .unwrap();

    processing.start();
}
