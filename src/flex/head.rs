extern crate byteorder;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Head {
    pub preamble: Vec<char>,
    pub idr     : u32,
    pub ids     : u32,
    pub size    : u16,
    pub csd     : u8,
    pub csp     : u8
}

pub fn xor_sum(mut buffer: u32, mut length: u32) -> u32 {
    let mut temp_sum: u32 = 0;

    while length > 0 {
        buffer += 1;
        temp_sum = temp_sum ^ buffer;
        length -= 1;
    }

    temp_sum
}

pub fn new(data: &[u8]) -> Head {
    let mut head: Head = Head {
        preamble: vec![],
        csd: 0,
        csp: 0,
        idr: 0,
        ids: 0,
        size: 0
    };

    for i in 0..=3 {
        head.preamble.push(data[i] as char);
    }

    head.csd    = data[14];
    head.csp    = data[15];

    let mut data = Cursor::new(&data[4..14]);
    head.idr    = data.read_u32::<BigEndian>().unwrap();
    head.ids    = data.read_u32::<BigEndian>().unwrap();
    head.size   = data.read_u16::<BigEndian>().unwrap();

    head
}