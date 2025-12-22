use crate::raytracing::material::ambient::AmbientMaterial;
use crate::raytracing::material::color::MaterialColor;
use crate::raytracing::material::MaterialType;

pub fn shade_hit_with_material(material: &MaterialType) -> MaterialColor {
    match material
    {
        MaterialType::Ambient(ambient_material) => {
            shade_ambient_material(ambient_material)
        }
    }
}

fn shade_ambient_material(material: &AmbientMaterial) -> MaterialColor {
    material.get_color()
}