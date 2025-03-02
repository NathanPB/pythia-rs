mod config;
mod data;
mod io;
mod processing;
mod registry;
mod utils;

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

    let (_config, _args, config_file) = cfg_result.unwrap();
    println!(
        "Loaded configuration file from {}",
        config_file.canonicalize().ok().unwrap().display()
    );
}
