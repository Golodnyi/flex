#[derive(Debug)]
struct Head {
    preamble: Vec<char>,
    idr     : u32,
    ids     : u32,
    size    : u16,
    csd     : u8,
    csp     : u8
}