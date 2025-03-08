#![feature(mpmc_channel)]

mod config;
mod data;
mod processing;
mod registry;
mod sites;
mod utils;
mod workdir;

use crate::processing::ProcessingBuilder;
use crate::workdir::make_workdir;
use registry::{itself::init_itself, Registries};

fn main() {
    let cfg_result = config::init();
    if let Err(e) = cfg_result {
        println!("{}", e);
        return;
    }

    let mut registries = Registries::new();
    let namespace = init_itself(&mut registries).unwrap();
    println!("Initialized own resources on namespace \"{}\"", namespace);

    let (config, args, config_file) = cfg_result.unwrap();
    println!(
        "Loaded configuration file from {}",
        config_file.canonicalize().ok().unwrap().display()
    );

    let (workdir, temp_wd) =
        match make_workdir(&args.workdir, &args.keep_workdir, args.clear_workdir) {
            Ok(workdir) => workdir,
            Err(e) => {
                println!("Unable to validate working directory: {}", e);
                return;
            }
        };

    println!(
        "Initialized working directory at {}{}",
        workdir.display(),
        if temp_wd { " (temporary)" } else { "" }
    );

    let processing = ProcessingBuilder {
        config: &config,
        args: &args,
        workdir,
        default_namespace: &namespace,
        registries: &mut registries,
    }
    .build()
    .unwrap();

    processing.start();
}
