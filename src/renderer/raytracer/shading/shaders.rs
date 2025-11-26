use crate::raytracing::object::material::material_type::MaterialType;
use crate::raytracing::object::shader::Shader;
use crate::renderer::raytracer::shading::ambient_shader::AmbientShader;

pub struct RaytracerShaders;

impl RaytracerShaders {
    pub fn shader(material_type: &MaterialType) -> impl Shader {
        match material_type {
            MaterialType::Ambient(ambient) => AmbientShader::new(ambient.clone()),
        }
    }
}
