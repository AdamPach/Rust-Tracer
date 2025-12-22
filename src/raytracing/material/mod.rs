use crate::raytracing::material::ambient::{AmbientMaterial};

pub mod ambient;
pub mod color;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MaterialTypeId(i32);

impl MaterialTypeId {
    pub fn new(material_id: i32) -> Self {
        Self(material_id)
    }
}

pub enum MaterialType {
    Ambient(AmbientMaterial),
}