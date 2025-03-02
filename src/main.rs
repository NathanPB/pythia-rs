mod config;
mod data;
mod io;
mod processing;

fn main() {
    let cfg_result = config::init();
    if let Err(e) = cfg_result {
        println!("Unable to load config file: {}", e);
        return;
    }

    let (_config, _args, config_file) = cfg_result.unwrap();
    println!(
        "Loaded configuration file from {}",
        config_file.canonicalize().ok().unwrap().display()
    );
}
