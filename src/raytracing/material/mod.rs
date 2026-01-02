use crate::raytracing::material::ambient::AmbientMaterial;
use std::collections::HashMap;
use crate::raytracing::material::diffuse::DiffuseMaterial;
use crate::raytracing::material::emissive::EmissiveMaterial;

pub mod ambient;
pub mod color;
pub mod diffuse;
pub mod emissive;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct MaterialTypeId(i32);

impl MaterialTypeId {
    pub fn new(material_id: i32) -> Self {
        Self(material_id)
    }
}

pub enum MaterialType {
    Ambient(AmbientMaterial),
    Diffuse(DiffuseMaterial),
    Emissive(EmissiveMaterial),
}

pub struct MaterialsRegistry {
    next_id: i32,
    materials: HashMap<MaterialTypeId, MaterialType>,
}

impl MaterialsRegistry {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            materials: HashMap::new(),
        }
    }

    pub fn add(&mut self, material: impl Into<MaterialType>) -> MaterialTypeId {
        let id = MaterialTypeId::new(self.next_id);
        self.materials.insert(id, material.into());
        self.next_id += 1;
        id
    }

    pub fn get(&self, material_id: MaterialTypeId) -> Option<&MaterialType> {
        self.materials.get(&material_id)
    }
}
