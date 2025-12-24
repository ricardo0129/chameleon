mod config;

use crate::config::parse;

fn main() {
    println!("Hello, world!");
    parse::hello();
}
