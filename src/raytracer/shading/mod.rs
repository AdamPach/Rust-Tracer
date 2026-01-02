use crate::raytracing::material::MaterialType;
use crate::raytracing::material::ambient::AmbientMaterial;
use crate::raytracing::material::color::MaterialColor;
use crate::raytracing::material::diffuse::DiffuseMaterial;
use crate::raytracing::material::emissive::EmissiveMaterial;
use crate::raytracing::{RayHit, Scene};

pub fn shade_hit(ray_hit: &RayHit, scene: &Scene) -> Option<MaterialColor> {
    let Some(material) = scene.get_material(ray_hit.material_id()) else {
        return None;
    };

    match material {
        MaterialType::Ambient(ambient_material) => Some(shade_ambient_material(ambient_material)),
        MaterialType::Diffuse(diffuse_material) => {
            Some(shade_diffuse_material(diffuse_material, scene))
        }
        MaterialType::Emissive(emissive_material) => {
            Some(shade_emissive_material(emissive_material))
        }
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
