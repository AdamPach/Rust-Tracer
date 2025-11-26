use crate::raytracing::object::material::color::Color;
use crate::raytracing::object::material::material_type::MaterialType;
use crate::raytracing::RayHit;
use crate::renderer::raytracer::shading::ambient_shader::AmbientShader;
pub trait Shader {
    fn shade(&self, hit: RayHit) -> Color;
}


pub struct RaytracerShaders;

impl RaytracerShaders {
    pub fn shader(material_type: &MaterialType) -> impl Shader {
        match material_type {
            MaterialType::Ambient(ambient) => AmbientShader::new(ambient.clone()),
        }
    }
}
