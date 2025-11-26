use crate::raytracing::object::material::ambient::AmbientMaterial;

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
