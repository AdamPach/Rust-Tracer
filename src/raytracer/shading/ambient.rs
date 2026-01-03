use crate::raytracing::material::ambient::AmbientMaterial;
use crate::raytracing::material::color::Color;

pub fn ambient_material_shader(material: &AmbientMaterial) -> Color {
    material.get_color()
}
