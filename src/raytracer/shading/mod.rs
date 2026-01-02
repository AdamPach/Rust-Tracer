use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::matrix3x3::Matrix3x3;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::material::MaterialType;
use crate::raytracing::material::ambient::AmbientMaterial;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::material::diffuse::DiffuseMaterial;
use crate::raytracing::material::emissive::EmissiveMaterial;
use crate::raytracing::{Ray, RayHit, Scene};
use rand::Rng;
use rand::prelude::ThreadRng;
use std::f64::consts::PI;
use crate::core::geometry::point::Point;

pub struct TracingContext<'a> {
    scene: &'a Scene,
    ray_hit: Option<&'a RayHit>,
    random: ThreadRng,
    depth: u32,
    max_depth: u32,
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

pub fn trace_ray(ray: Ray, ctx: TracingContext) -> MaterialColor {
    if ctx.depth >= ctx.max_depth {
        return MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));
    }

    let Some(ray_hit) = ctx.scene.find_intersection(ray) else {
        return MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));
    };

    let Some(material) = ctx.scene.get_material(ray_hit.material_id()) else {
        panic!("Material not found for id {:?}", ray_hit.material_id());
    };

    let ctx = TracingContext {
        ray_hit: Some(&ray_hit),
        ..ctx
    };

    match material {
        MaterialType::Ambient(ambient_material) => shade_ambient_material(ambient_material),
        MaterialType::Diffuse(diffuse_material) => shade_diffuse_material(diffuse_material, ctx),

        MaterialType::Emissive(emissive_material) => shade_emissive_material(emissive_material),
    }
}

fn shade_ambient_material(material: &AmbientMaterial) -> MaterialColor {
    material.get_color()
}

fn shade_diffuse_material(
    material: &DiffuseMaterial,
    mut ctx: TracingContext,
) -> MaterialColor {
    let r1: f64 = ctx.random.random();
    let r2: f64 = ctx.random.random();

    let phi = 2.0 * PI * r1;

    let x = r2.sqrt() * phi.cos();
    let y = r2.sqrt() * phi.sin();
    let z = (1.0 - r2).sqrt();
    
    let normal = ctx.ray_hit.unwrap().interpolated_normal();

    let o1 = orthogonal_vector(&normal);
    let o2 = normal.cross(&o1).norm();

    let sampled_direction = Matrix3x3::from_columns(o1, o2, normal)
        .mul_by_vec3(&Vector3::new(X::new(x), Y::new(y), Z::new(z)))
        .norm();

    let cos_theta = normal.dot(&sampled_direction);

    let pdf = cos_theta / PI;

    if cos_theta <= 1e-18 {
        return MaterialColor::new(R::new(0.0), G::new(0.0), B::new(0.0), A::new(1.0));
    }

    let albedo = material.albedo();

    let brdf = MaterialColor::new(
        R::new(albedo.r().get() / PI as f32),
        G::new(albedo.g().get() / PI as f32),
        B::new(albedo.b().get() / PI as f32),
        A::new(1.0),
    );

    let ray_hit = ctx.ray_hit.unwrap();

    let hit_point = Point::new(
        X::new(
            ray_hit.ray().origin().x().get()
                + ray_hit.ray().direction().get_x().get() * ray_hit.distance(),
        ),
        Y::new(
            ray_hit.ray().origin().y().get()
                + ray_hit.ray().direction().get_y().get() * ray_hit.distance(),
        ),
        Z::new(
            ray_hit.ray().origin().z().get()
                + ray_hit.ray().direction().get_z().get() * ray_hit.distance(),
        ),
    );

    let new_ray = Ray::new(hit_point, sampled_direction, 1e-6);
    
    ctx.depth += 1;
    
    let incoming_radiance = trace_ray(new_ray, ctx);
    
    MaterialColor::new(
        R::new(
            brdf.r().get() * incoming_radiance.r().get() * cos_theta as f32 / pdf as f32,
        ),
        G::new(
            brdf.g().get() * incoming_radiance.g().get() * cos_theta as f32 / pdf as f32,
        ),
        B::new(
            brdf.b().get() * incoming_radiance.b().get() * cos_theta as f32 / pdf as f32,
        ),
        A::new(1.0),
    )
}

fn shade_emissive_material(material: &EmissiveMaterial) -> MaterialColor {
    material.emission()
}

fn orthogonal_vector(n: &Vector3) -> Vector3 {
    if n.get_x().get().abs() > n.get_z().get().abs() {
        Vector3::new(
            X::new(n.get_y().get()),
            Y::new(-n.get_x().get()),
            Z::new(0.0),
        )
        .norm()
    } else {
        Vector3::new(
            X::new(0.0),
            Y::new(n.get_z().get()),
            Z::new(-n.get_y().get()),
        )
        .norm()
    }
}
