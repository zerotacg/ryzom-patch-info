#[derive(Debug)]
pub struct Header {
    pub version: u32,
    pub total_size: u32,
    pub token_count: u32,
    pub arg_count: u32,
    pub string_count: u32,
    pub strings_size: u32,
}
