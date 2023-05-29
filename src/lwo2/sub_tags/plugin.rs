use crate::binrw_helpers::lwo_null_string;
use binrw::{binread, PosValue};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct PluginServerNameAndData {
    #[br(temp)]
    start_pos: PosValue<()>,
    #[br(parse_with = lwo_null_string)]
    pub server_name: String,
    pub flags: u16,
    #[br(temp)]
    end_pos: PosValue<()>,
    #[br(count = length as u64 - (end_pos.pos - start_pos.pos))]
    pub parameters: Vec<u8>,
}
