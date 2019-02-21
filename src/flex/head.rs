#[derive(Debug)]
pub struct Head {
    pub preamble: Vec<char>,
    pub idr     : u32,
    pub ids     : u32,
    pub size    : u16,
    pub csd     : u8,
    pub csp     : u8
}