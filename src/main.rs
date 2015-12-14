extern crate bitcoin;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::SocketResponse;

use bitcoin::network::message_blockdata::GetBlocksMessage;
use bitcoin::util::hash::Sha256dHash;
use std::fmt::Debug;

use std::sync::mpsc::{channel, Receiver};

fn main() {

    
        let (recv_tx, recv_rx) = channel();
        
        let mut sock = Socket::new(Network::Bitcoin);
        let mut ret_sock = Socket::new(Network::Bitcoin);
        
        sock.connect("127.0.0.1", 8333);
        
        let locator_hashes : Vec<Sha256dHash> = Vec::new();
        let stop_hash = Sha256dHash::from_data(&[]);
                       
        let gbm = GetBlocksMessage::new(locator_hashes, stop_hash);
        
        let nm = NetworkMessage::GetBlocks(gbm);
        
        let result = sock.send_message(nm);
        
        // Message loop
        thread::spawn(move || {
            let mut handshake_complete = false;
            let mut sock = sock;
            loop {
                // Receive new message
                match sock.receive_message() {
                    Ok(payload) => {
                        // React to any network messages that affect our state.
                        if let NetworkMessage::Verack = payload {
                            // Make an exception for verack since there is no response required
                            // TODO: when the timeout stuff in std::io::net::tcp is sorted out we should
                            // actually time out if the verack doesn't come in in time
                            if handshake_complete {
                                println!("Received second verack (peer is misbehaving)");
                            } else {
                                handshake_complete = true;
                            }
                        };
                        println!("hello {:?}",payload);
                        // We have to pass the message to the main thread for processing,
                        // unfortunately, because sipa says we have to handle everything
                        // in order.
                        recv_tx.send(SocketResponse::MessageReceived(payload)).unwrap();
                    }
                    Err(e) => {
                        println!("error {:?}",e);
                        // On failure we send an error message to the main thread, along with
                        // a channel to receive an acknowledgement that we may tear down this
                        // thread. (If we simply exited immediately, the channel would be torn
                        // down and the main thread would never see the error message.)
                        let (tx, rx) = channel();
                        recv_tx.send(SocketResponse::ConnectionFailed(e, tx)).unwrap();
                        rx.recv().unwrap();
                        break;
                    }
                }
            }
        });
    sleep(Duration::from_millis(10000));
}