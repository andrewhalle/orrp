//! # orrp - Open a Random Rust Page
//!
//! Utility for browsing *The Rust Programming Language*, the Rust
//! Standard Library, and any other sources of Rust information
//! you'd like to become more familiar with!

extern crate orrp;

use std::env;

fn main() {
    let args = env::args();
    let config = orrp::Config::new(args);
    if let Err(e) = orrp::run(config) {
        println!("Unable to perform the requested action");
    }
}
