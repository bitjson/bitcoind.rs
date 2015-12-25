// use node::db::save_blocks;
// use node::db::read_blocks;
use node::db::DataStore;
use node::peer::connect;

use std::thread;
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

use bitcoin::util::hash::Sha256dHash;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message_blockdata::GetBlocksMessage;
use bitcoin::network::message_blockdata::Inventory;

use rustc_serialize::hex::FromHex;

pub fn start() {

    let path = Path::new("./data");
    let datastore = DataStore::new(path);

    datastore.read_blocks();

    let wait_time = Duration::from_secs(10);

    let (tx_daemon, rx_daemon) = channel::<Vec<Inventory>>();

    let mut socket = connect("127.0.0.1", 8333, tx_daemon);


    thread::spawn(move || {
        loop {
            thread::sleep(wait_time);
            match rx_daemon.recv() {
                Ok(payload) => {
                    println!("recv inv thru tunnl {:?}", payload.len());

                    // for inv in payload {
                    //     println!("inv iter {:?}", inv);
                    // }
                    datastore.save_blocks(payload);
                }
                Err(e) => {
                    println!("getblocks error {:?}", e);
                }
            }
        }
    });


    let sha = Sha256dHash::from_hex("000000001695f1cae23b2b7f9c4879f210706a42d9d9c96146fcc66c6e87\
                                     b2c2")
                  .unwrap();
    let sha2 = Sha256dHash::from_hex("00000000000067bf5f3ab6ba97e33bd6488155aafd0bc449084f8a854ce\
                                      41594")
                   .unwrap();

    println!("sha1 {:?}", sha);
    println!("sha2 {:?}", sha2);


    let mut locator_hashes: Vec<Sha256dHash> = Vec::new();
    locator_hashes.push(sha);
    let stop_hash = sha2;

    let a = GetBlocksMessage::new(locator_hashes, stop_hash);

    let network_message = NetworkMessage::GetBlocks(a);

    thread::sleep(wait_time);
    println!("sending getblocks sent ");

    match socket.send_message(network_message) {
        Ok(payload) => {
            println!("getblocks sent {:?} ", payload);
        }
        Err(e) => {
            println!("getblocks error {:?}", e);
        }
    }
}
