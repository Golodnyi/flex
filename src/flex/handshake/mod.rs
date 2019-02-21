use super::head;

#[derive(Debug)]
struct Handshake {
    head            : head::Head,
    prefix          : Vec<char>,
    protocol        : u8,
    protocol_version: u8,
    struct_version  : u8,
    data_size       : u8,
    bitfield        : Vec<u8>
}

pub fn new() {
    println!("handshake!");
}
