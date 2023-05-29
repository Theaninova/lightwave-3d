use crate::binrw_helpers::{lwo_null_string, until_size_limit};
use crate::iff::SubChunk;
use crate::lwo2::sub_tags::VectorEnvelope;
use crate::lwo2::vx;
use binrw::{binread};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct TextureMapping {
    #[br(parse_with = until_size_limit(length as u64))]
    pub attributes: Vec<TextureMappingSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum TextureMappingSubChunk {
    #[br(magic(b"CNTR"))]
    Center(SubChunk<VectorEnvelope>),
    #[br(magic(b"SIZE"))]
    Size(SubChunk<VectorEnvelope>),
    #[br(magic(b"ROTA"))]
    Rotation(SubChunk<VectorEnvelope>),
    #[br(magic(b"OREF"))]
    ReferenceObject(SubChunk<ReferenceObject>),
    #[br(magic(b"FALL"))]
    Falloff(SubChunk<Falloff>),
    #[br(magic(b"CSYS"))]
    CoordinateSystem(SubChunk<CoordinateSystem>),
}

#[binread]
#[br(repr = u16, import(_length: u32))]
#[derive(Debug)]
pub enum CoordinateSystem {
    ObjectCoordinates = 0,
    WorldCoordinates = 1,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ReferenceObject {
    #[br(parse_with = lwo_null_string)]
    pub object_name: String,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Falloff {
    pub kind: FalloffType,
    pub vector: [f32; 3],
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum FalloffType {
    Cubic = 0,
    Spherical = 1,
    LinearX = 2,
    LinearY = 3,
    LinearZ = 4,
}
