use crate::core::render::color::Color;
use crate::raytracing::material::emissive::EmissiveMaterial;

pub fn emissive_material_shader(material: &EmissiveMaterial) -> Color {
    material.emission()
}
