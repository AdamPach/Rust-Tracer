use crate::core::render::color::{A, B, Color, G, R};
use crate::rendering::shading::sampling::cos_weighted_hemisphere_sample;
use crate::rendering::shading::{TracingContext, trace_ray};
use crate::raytracing::Ray;
use crate::raytracing::material::diffuse::DiffuseMaterial;
use std::f64::consts::PI;

pub fn diffuse_material_shader(material: &DiffuseMaterial, mut ctx: TracingContext) -> Color {
    let (ray_hit, geometry) = ctx.hit.unwrap();

    let normal = geometry.interpolate_normal(ray_hit.barycentric());

    let sample = cos_weighted_hemisphere_sample(&normal, &mut ctx.random);

    if sample.cos_theta <= 1e-18 {
        return Color::new(R::new(0.0), G::new(0.0), B::new(0.0), A::new(1.0));
    }

    let albedo = material.albedo();

    let brdf = Color::new(
        albedo.r() / PI,
        albedo.g() / PI,
        albedo.b() / PI,
        A::new(1.0),
    );

    let hit_point = ray_hit.hit_point();

    let incoming_radiance = trace_ray(
        Ray::new(hit_point, sample.sampled_direction, 1e-6),
        TracingContext {
            depth: ctx.depth + 1,
            hit: None,
            ..ctx
        },
    );

    Color::new(
        brdf.r() * incoming_radiance.r() * sample.cos_theta / sample.pdf,
        brdf.g() * incoming_radiance.g() * sample.cos_theta / sample.pdf,
        brdf.b() * incoming_radiance.b() * sample.cos_theta / sample.pdf,
        A::new(1.0),
    )
}
