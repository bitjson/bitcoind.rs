use node::db::test_ldb;
use node::peer::connect;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use bitcoin::util::hash::Sha256dHash;

use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::SocketResponse;
use bitcoin::network::message_network::VersionMessage;
use bitcoin::network::message_blockdata::GetBlocksMessage;
use bitcoin::network::message_blockdata::Inventory;

extern crate rustc_serialize;

use self::rustc_serialize::hex::FromHex;

pub fn start() {
    test_ldb();


    let (txDaemon, rxDaemon) = channel::<Vec<Inventory>>();

    let mut socket = connect("127.0.0.1", 8333, txDaemon);


    thread::spawn( move || {
        loop {
            match rxDaemon.recv() {
                Ok(payload) => {
                    println!("recv inv thru tunnl {:?}", payload.len());

                    for inv in payload {
                        println!("inv iter {:?}", inv);
                    }
                },
                Err(e) => {
                    println!("getblocks error {:?}",e);
                }
            }
        }
    });



    let waitTime = Duration::from_secs(10);


    let hash = "00000000d14451e1f057e65b076f3aacb30da7cf0fde3d43419b08a207b94bfd".from_hex().unwrap();
    let hash2 = "00000000c2d671c26e69d1baede1e4e6891a63dca35457d58175d55f98720d6c".from_hex().unwrap();

        let sha = Sha256dHash::from_data(&hash);
            let sha2 = Sha256dHash::from_data(&hash2);

    let mut locator_hashes : Vec<Sha256dHash> = Vec::new();
    locator_hashes.push(sha);
    let stop_hash = sha2;

    let a = GetBlocksMessage::new(locator_hashes, stop_hash);

    let network_message = NetworkMessage::GetBlocks(a);

    thread::sleep(waitTime);
        println!("sending getblocks sent ");

    match socket.send_message(network_message) {
        Ok(payload) => {
            println!("getblocks sent {:?} ", payload);
        },
        Err(e) => {
            println!("getblocks error {:?}",e);
        }
    }
}
