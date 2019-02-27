extern crate byteorder;

use super::head;
use std::thread;
use tokio::net::tcp::Incoming;
use serialport::SerialPort;
use tokio::prelude::*;

#[derive(Debug)]
struct Handshake {
    head    : head::Head,
    prefix  : Vec<char>,
    imei    : Vec<char>
}

pub fn socket(stream: Incoming) {
    let (reader, writer) = stream.split();
    let handshake = fill_data(&reader);
}

pub fn com_port(stream: &SerialPort) {
    thread::spawn(|| {
    });
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
