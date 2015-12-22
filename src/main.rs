extern crate bitcoin;
extern crate leveldb;

pub mod node;

use node::peer::connect;
use node::db::test_ldb;
use std::io;
use std::io::prelude::*;

fn main() {
    test_ldb();

    //connect("127.0.0.1", 8333);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
