use crate::raytracing::intersection::ray_hit::RayHit;
use crate::raytracing::object::material::color::Color;

pub trait Shader {
    fn shade(&self, hit: RayHit) -> Color;
}
