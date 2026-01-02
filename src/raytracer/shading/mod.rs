use crate::raytracing::material::MaterialType;
use crate::raytracing::material::ambient::AmbientMaterial;
use crate::raytracing::material::color::MaterialColor;
use crate::raytracing::material::diffuse::DiffuseMaterial;
use crate::raytracing::material::emissive::EmissiveMaterial;
use crate::raytracing::Scene;

pub fn shade_hit_with_material(material: &MaterialType, scene: &Scene) -> MaterialColor {
    match material {
        MaterialType::Ambient(ambient_material) => shade_ambient_material(ambient_material),
        MaterialType::Diffuse(diffuse_material) => shade_diffuse_material(diffuse_material, scene),
        MaterialType::Emissive(emissive_material) => shade_emissive_material(emissive_material),
    }
}

fn shade_ambient_material(material: &AmbientMaterial) -> MaterialColor {
    material.get_color()
}

fn shade_diffuse_material(material: &DiffuseMaterial, scene: &Scene) -> MaterialColor {
    // Placeholder for diffuse shading logic
    material.albedo()
}

fn shade_emissive_material(material: &EmissiveMaterial) -> MaterialColor {
    material.emission()
}