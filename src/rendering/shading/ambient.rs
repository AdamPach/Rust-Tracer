use crate::core::render::color::Color;
use crate::raytracing::material::ambient::AmbientMaterial;

pub fn ambient_material_shader(material: &AmbientMaterial) -> Color {
    material.get_color()
}
