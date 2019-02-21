extern crate byteorder;

use super::head;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Write;
use std::io::Read;
use std::net::{TcpListener, Shutdown};
use std::thread;

#[derive(Debug)]
struct Handshake {
    head    : head::Head,
    prefix  : Vec<char>,
    imei    : Vec<char>
}

pub fn new(listener: TcpListener) {

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut data = vec![0, 34];

            while match stream.read(&mut data) {
                Ok(size) => {
                    // TODO: check return byte size
                    let handshake = fill_data(&data);
                    println!("{:?}", handshake);                    

                    true
                },
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap();

                    false
                }
            } {}
        });
    }
}

fn fill_data(data: &[u8]) -> Handshake {
    let mut handshake: Handshake = Handshake {
        head: head::new(&data[0..=15]),
        prefix: vec![],
        imei: vec![]
    };

    for i in 16..=18 {
        handshake.prefix.push(data[i] as char);
    }

    for i in 18..=33 {
        handshake.imei.push(data[i] as char);
    }

    handshake
}
