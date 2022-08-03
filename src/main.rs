mod lib;
use lib::Config;
use std::env;
use std::fs;
use std::process;
fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let variations = lib::split(&config);
    fs::write(&config.output_file, lib::convert(&variations)).unwrap();
}
