extern crate bitcoin;

use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::SocketResponse;

use std::thread;
use bitcoin::network::message_network::VersionMessage;

pub fn connect(host: &'static str, port: u16) {

    thread::spawn( move || {
        
        let mut socket = Socket::new(Network::Testnet);
        
        match socket.connect(host, port) {
            Ok(()) => {
                
                match VersionMessage::new(14213, socket.clone(), 12421, 2048) {
                    Ok(version_message) => {
                        println!("version_message {:?}",version_message);
                        let network_message = NetworkMessage::Version(version_message);
                        
                        match socket.send_message(network_message) {
                            Ok(()) => {
                                
                                match socket.receive_message() {
                                    Ok(payload) => {
                                        println!("hello {:?}",payload);
                                    }
                                    Err(e) => {
                                        println!("error {:?}",e);
                                    }
                                }
                                
                            }
                            Err(e) => {
                                println!("error {:?}",e);
                            }
                        }
                        
                    }
                    Err(e) => {
                        println!("error {:?}", e);
                    }
                    
                }
                
                
            }
            Err(e) => {
                println!("error {:?}", e);
            }
        }
        
    });
    
}