extern crate bitcoin;
extern crate rand;

use std::thread;
use std::time::Duration;
use std::u64;
use std::sync::mpsc::{channel, Sender, Receiver};
use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};

use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::SocketResponse;
use bitcoin::network::message_network::VersionMessage;
use bitcoin::network::message_blockdata::Inventory;

pub fn connect(host: &'static str, port: u16, tx_daemon: Sender<Vec<Inventory>>) -> Socket{



    let mut socket = Socket::new(Network::Testnet);

    match socket.connect(host, port) {
        Ok(()) => recv_loop(socket.clone(), tx_daemon),
        Err(e) => {
            println!("error {:?}", e);
        }
    }

    fn send_ping(tx: Sender<NetworkMessage>) {

        let ping_interval = Duration::from_secs(10);
        thread::sleep(ping_interval);

        let mut rng = rand::thread_rng();
        let between = Range::new(u64::MIN, u64::MAX);
        let nonce = between.ind_sample(&mut rng);

        let ping_message = NetworkMessage::Ping(nonce);
        println!("sending ping");

        match tx.send(ping_message) {
            Ok(()) => {
                println!("ping sent with nonce {:?}", nonce);
            },
            Err(e) => {
                println!("error {:?}",e);
            }
        }
    }

    fn send_pong(socket: &mut Socket, nonce: u64) {
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

    fn sender(mut socket: Socket, rx: Receiver<NetworkMessage>) {
        loop {
            let message = rx.recv().unwrap();
            match socket.send_message(message) {
                Ok(()) => {
                    println!("sent");
                },
                Err(e) => {
                    println!("error {:?}",e);
                }
            }
        }
    }


    fn pinger(mut socket: Socket, rx: Receiver<NetworkMessage>) {

        let ping_interval = Duration::from_secs(120);
        let mut rng = rand::thread_rng();
        let between = Range::new(u64::MIN, u64::MAX);

        loop {

            let nonce = between.ind_sample(&mut rng);
            let ping_message = NetworkMessage::Ping(nonce);

            match socket.send_message(ping_message) {
                Ok(()) => {
                    println!("sent ping ");
                },
                Err(e) => {
                    println!("error {:?}",e);
                }
            }


            thread::sleep(ping_interval);

            match rx.try_recv() {
                Ok(payload) => {
                    println!("pong received by pinger {:?} ", payload);
                },
                Err(e) => {
                    println!("error {:?}",e);
                    break;
                }
            }


        }

        println!("Peer didn't respond to ping");
    }

    fn receiver(mut socket: Socket, tx: Sender<NetworkMessage>, tx_daemon: Sender<Vec<Inventory>>) {
        loop {
            match socket.receive_message() {
                Ok(payload) => {
                    // println!("received {:?}",payload);

                    let message : NetworkMessage = payload;
                    match message {
                        NetworkMessage::Version(nonce) => {
                            println!("ping nonce {:?}", nonce);
                        }
                        NetworkMessage::Verack => {
                            println!("verack");
                        }
                        NetworkMessage::Ping(nonce) => {
                            println!("ping nonce {:?}", nonce);
                            send_pong(&mut socket, nonce);
                        }
                        NetworkMessage::Addr(addr) => {
                            println!("addr {:?}", addr);
                        }
                        NetworkMessage::Inv(inv) => {
                            tx_daemon.send(inv);
                        }
                        NetworkMessage::GetData(inv) => {
                            println!("GetData {:?}", inv);
                        }
                        NetworkMessage::NotFound(inv) => {
                            println!("NotFound {:?}", inv);
                        }
                        NetworkMessage::GetBlocks(inv) => {
                            println!("GetBlocks {:?}", inv);
                        }
                        NetworkMessage::GetHeaders(inv) => {
                            println!("GetHeaders {:?}", inv);
                        }
                        NetworkMessage::Tx(inv) => {
                            println!("Tx {:?}", inv);
                        }
                        NetworkMessage::Block(inv) => {
                            println!("Block {:?}", inv);
                        }
                        NetworkMessage::Headers(inv) => {
                            println!("Headers {:?}", inv);
                        }
                        NetworkMessage::Pong(inv) => {
                            println!("Pong {:?}", inv);
                            tx.send(message);
                        }
                        NetworkMessage::MemPool => {
                            println!("MemPool");
                        }
                    }
                }
                Err(e) => {
                    println!("error {:?}",e);
                }
            }

            let ping_interval = Duration::from_secs(1);
            thread::sleep(ping_interval);
        }
    }

    fn send_version_message(mut socket: Socket, version_message: VersionMessage, tx_daemon: Sender<Vec<Inventory>>) {
        let network_message = NetworkMessage::Version(version_message);
        match socket.send_message(network_message) {
            Ok(()) => {
                let (txMessageReceiver, rxMessageReceiver) = channel::<NetworkMessage>();
                // let (txMessageSender, rxMessageSender) = channel::<NetworkMessage>();

                let sender_socket = socket.clone();
                let receiver_socket = socket.clone();

                thread::spawn( move || {
                    receiver(receiver_socket, txMessageReceiver, tx_daemon);
                });

                thread::spawn( move || {
                    pinger(sender_socket, rxMessageReceiver);
                });

                // thread::spawn( move || {
                //     loop {tx_daemon: Sender<Vec<Inventory>>
                //         send_pong(&mut sender_socket.clone(), 2353);
                //     }
                // });

            }
            Err(e) => {
                println!("error {:?}",e);
            }
        }
    }

    fn recv_loop(socket : Socket, tx_daemon: Sender<Vec<Inventory>>) {

        thread::spawn( move || {

                match VersionMessage::new(14213, socket.clone(), 12421, 2048) {
                    Ok(version_message) => send_version_message(socket, version_message, tx_daemon),
                    Err(e) => {
                        println!("error {:?}", e);
                    }
                }


        });

    }

    return socket;

}
