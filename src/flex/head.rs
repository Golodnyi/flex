#[derive(Debug)]
pub struct Head {
    pub preamble: Vec<char>,
    pub idr     : u32,
    pub ids     : u32,
    pub size    : u16,
    pub csd     : u8,
    pub csp     : u8
}

pub fn xor_sum(buffer: u32, length: u32) -> u32 {
    let temp_sum: u32 = 0;

    while length > 0 {
        buffer += 1;
        let temp_sum: u32 = temp_sum ^ buffer;
        length -= 1;
    }

    temp_sum
}