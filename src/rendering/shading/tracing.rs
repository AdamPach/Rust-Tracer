use crate::core::render::color::{A, B, Color, G, R};
use crate::rendering::shading::ambient::ambient_material_shader;
use crate::rendering::shading::emissive::emissive_material_shader;
use crate::rendering::shading::lambert_diffuse::diffuse_material_shader;
use crate::raytracing::material::MaterialType;
use crate::raytracing::{GeometryPrimitive, Ray, RayHit, Scene};
use rand::prelude::ThreadRng;

pub struct TracingContext<'a> {
    pub scene: &'a Scene,
    pub hit: Option<(&'a RayHit, GeometryPrimitive<'a>)>,
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
            hit: None,
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

    let geometry = ctx.scene.get_geometry(&ray_hit.geometry_index());

    let Some(material) = ctx.scene.get_material(geometry.material_id()) else {
        panic!("Material not found for id {:?}", geometry.material_id());
    };

    let ctx = TracingContext {
        hit: Some((&ray_hit, geometry)),
        ..ctx
    };

    match material {
        MaterialType::Ambient(ambient_material) => ambient_material_shader(ambient_material),
        MaterialType::Diffuse(diffuse_material) => diffuse_material_shader(diffuse_material, ctx),
        MaterialType::Emissive(emissive_material) => emissive_material_shader(emissive_material),
    }
}
