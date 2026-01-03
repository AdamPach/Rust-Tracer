use crate::raytracer::shading::ambient::ambient_material_shader;
use crate::raytracer::shading::emissive::emissive_material_shader;
use crate::raytracer::shading::lambert_diffuse::diffuse_material_shader;
use crate::raytracing::material::MaterialType;
use crate::raytracing::material::color::{A, B, Color, G, R};
use crate::raytracing::{Ray, RayHit, Scene};
use rand::prelude::ThreadRng;

pub struct TracingContext<'a> {
    pub scene: &'a Scene,
    pub ray_hit: Option<&'a RayHit>,
    pub random: ThreadRng,
    pub depth: u32,
    pub max_depth: u32,
}

impl<'a> TracingContext<'a> {
    pub fn new(scene: &'a Scene, max_depth: u32) -> Self {
        Self {
            scene,
            random: rand::rng(),
            depth: 0,
            max_depth,
            ray_hit: None,
        }
    }
}

pub fn trace_ray(ray: Ray, ctx: TracingContext) -> Color {
    if ctx.depth >= ctx.max_depth {
        return Color::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));
    }

    let Some(ray_hit) = ctx.scene.find_intersection(ray) else {
        return Color::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));
    };

    let Some(material) = ctx.scene.get_material(ray_hit.material_id()) else {
        panic!("Material not found for id {:?}", ray_hit.material_id());
    };

    let ctx = TracingContext {
        ray_hit: Some(&ray_hit),
        ..ctx
    };

    match material {
        MaterialType::Ambient(ambient_material) => ambient_material_shader(ambient_material),
        MaterialType::Diffuse(diffuse_material) => diffuse_material_shader(diffuse_material, ctx),
        MaterialType::Emissive(emissive_material) => emissive_material_shader(emissive_material),
    }
}
