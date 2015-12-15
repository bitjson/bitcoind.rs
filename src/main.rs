extern crate bitcoin;

pub mod node;

use node::peer::connect;
use std::io;
use std::io::prelude::*;

fn main() {
    connect("127.0.0.1", 8333);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}