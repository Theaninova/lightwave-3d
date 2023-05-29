use crate::binrw_helpers::{lwo_null_string, until_size_limit};
use crate::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use binrw::{binread, PosValue};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct SurfaceDefinition {
    #[br(temp)]
    start_pos: PosValue<()>,
    #[br(parse_with = lwo_null_string)]
    pub name: String,
    #[br(parse_with = lwo_null_string)]
    pub source: String,
    #[br(temp)]
    end_pos: PosValue<()>,
    #[br(parse_with = until_size_limit(length as u64 - (end_pos.pos - start_pos.pos)))]
    pub attributes: Vec<SurfaceParameterSubChunk>,
}
