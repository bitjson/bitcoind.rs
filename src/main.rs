extern crate bitcoin;
extern crate lmdb;
extern crate rustc_serialize;
extern crate rand;

use std::io;
use std::io::prelude::*;

mod node;

fn main() {

    node::daemon::start();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
