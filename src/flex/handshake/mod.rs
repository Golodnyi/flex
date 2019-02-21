use super::head;

#[derive(Debug)]
struct Handshake {
    head    : head::Head,
    prefix  : Vec<char>,
    imei    : Vec<char>
}

pub fn new() {
    println!("handshake!");
}
