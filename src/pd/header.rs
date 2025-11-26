#[derive(Debug)]
pub struct Header {
    pub(crate) version: u32,
    pub(crate) total_size: u32,
    pub(crate) token_count: u32,
    pub(crate) arg_count: u32,
    pub(crate) string_count: u32,
    pub(crate) strings_size: u32,
}
