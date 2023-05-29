use crate::binrw_helpers::{lwo_null_string, until_size_limit_with};
use binrw::binread;

/// Lists the tag strings that can be associated with polygons by the PTAG chunk.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct TagStrings {
    #[br(parse_with = until_size_limit_with(length as u64, lwo_null_string))]
    pub tag_strings: Vec<String>,
}
