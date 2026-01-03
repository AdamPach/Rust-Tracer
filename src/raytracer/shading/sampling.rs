use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::matrix3x3::Matrix3x3;
use crate::core::geometry::vector::Vector3;
use rand::Rng;
use rand::prelude::ThreadRng;

pub struct CosineWeightedHemisphereSample {
    pub sampled_direction: Vector3,
    pub cos_theta: f64,
    pub pdf: f64,
}

pub fn cos_weighted_hemisphere_sample(
    normal: &Vector3,
    random: &mut ThreadRng,
) -> CosineWeightedHemisphereSample {
    let r1: f64 = random.random();
    let r2: f64 = random.random();

    let phi = 2.0 * std::f64::consts::PI * r1;

    let x = r2.sqrt() * phi.cos();
    let y = r2.sqrt() * phi.sin();
    let z = (1.0 - r2).sqrt();

    let o1 = orthogonal_vector(&normal);
    let o2 = normal.cross(&o1).norm();

    let sampled_direction = Matrix3x3::from_columns(o1, o2, normal.clone())
        .mul_by_vec3(&Vector3::new(X::new(x), Y::new(y), Z::new(z)))
        .norm();

    let cos_theta = sampled_direction.dot(&normal);

    let pdf = cos_theta / std::f64::consts::PI;

    CosineWeightedHemisphereSample {
        sampled_direction,
        cos_theta,
        pdf,
    }
}

fn orthogonal_vector(n: &Vector3) -> Vector3 {
    if n.x().get().abs() > n.z().get().abs() {
        Vector3::new(X::new(n.y().get()), Y::new(-n.x().get()), Z::new(0.0)).norm()
    } else {
        Vector3::new(X::new(0.0), Y::new(n.z().get()), Z::new(-n.y().get())).norm()
    }
}
