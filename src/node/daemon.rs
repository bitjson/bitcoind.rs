use node::db::test_ldb;
use node::peer::connect;

use std::thread;
use std::time::Duration;

use bitcoin::util::hash::Sha256dHash;

use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::SocketResponse;
use bitcoin::network::message_network::VersionMessage;
use bitcoin::network::message_blockdata::GetBlocksMessage;

extern crate rustc_serialize;

use self::rustc_serialize::hex::FromHex;

pub fn start() {
    test_ldb();

    let mut socket = connect("127.0.0.1", 8333);

    let waitTime = Duration::from_secs(10);


let hash = "00000000006bdf3d9399d565df7571eb773986babca04872d5048a838d5b14f9".from_hex().unwrap();

    let sha = Sha256dHash::from_data(&hash);

    let mut locator_hashes : Vec<Sha256dHash> = Vec::new();
    locator_hashes.push(sha);
    let stop_hash = sha;

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
