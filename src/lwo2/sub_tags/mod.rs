use crate::binrw_helpers::lwo_null_string;
use crate::lwo2::vx;
use binrw::binread;

pub mod blocks;
pub mod plugin;
pub mod surface_parameters;

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VectorEnvelope {
    pub base_color: [f32; 3],
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Name {
    #[br(parse_with = lwo_null_string)]
    pub name: String,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ValueEnvelope {
    pub value: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VxReference {
    #[br(parse_with = vx)]
    pub texture_image: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct EnableState {
    pub enable: u16,
}
