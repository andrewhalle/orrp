extern crate open;

use std::env;

pub struct Config {
}

impl Config {
    pub fn new(args: env::Args) -> Config {
        Config { }
    }
}

pub fn run(config: Config) -> Result<(), ()> {
    open::that("https://www.google.com").expect("Unable to open Google.");

    Ok(())
}
