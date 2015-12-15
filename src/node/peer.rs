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
        
        fn send_pong(mut socket: Socket, nonce: u64) {
            let pong_message = NetworkMessage::Pong(nonce);
            
            match socket.send_message(pong_message) {
                Ok(()) => {
                    println!("pong sent");
                },
                Err(e) => {
                    println!("error {:?}",e);
                }
            }
        }
        
        fn recieve_message(mut socket: Socket) {
            loop {
                match socket.receive_message() {
                    Ok(payload) => {
                        println!("received {:?}",payload);
                        match payload {
                            NetworkMessage::Version(nonce) => {
                                println!("ping nonce {:?}", nonce);
                            }
                            NetworkMessage::Verack => {
                                println!("verack");
                            }
                            NetworkMessage::Ping(nonce) => {
                                println!("ping nonce {:?}", nonce);
                                send_pong(socket.clone(), nonce);
                            }
                            NetworkMessage::Addr(addr) => {
                                println!("addr {:?}", addr);
                            }
                            NetworkMessage::Inv(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::GetData(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::NotFound(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::GetBlocks(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::GetHeaders(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::Tx(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::Block(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::Headers(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::Pong(inv) => {
                                println!("addr {:?}", inv);
                            }
                            NetworkMessage::MemPool => {
                                println!("addr");
                            }
                        }
                    }
                    Err(e) => {
                        println!("error {:?}",e);
                    }
                }
            }
        }
        
        fn send_version_message(mut socket: Socket, version_message: VersionMessage) {
            let network_message = NetworkMessage::Version(version_message);
            match socket.send_message(network_message) {
                Ok(()) => recieve_message(socket),
                Err(e) => {
                    println!("error {:?}",e);
                }
            }
        }
        
        fn on_connected(mut socket: Socket) {
            match VersionMessage::new(14213, socket.clone(), 12421, 2048) {
                Ok(version_message) => send_version_message(socket, version_message),
                Err(e) => {
                    println!("error {:?}", e);
                }
            }
        }
        
        match socket.connect(host, port) {
            Ok(()) => on_connected(socket),
            Err(e) => {
                println!("error {:?}", e);
            }
        }
        
    });
    
}