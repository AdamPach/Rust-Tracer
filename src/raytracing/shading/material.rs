use crate::raytracing::object::material::ambient::Ambient;
use crate::raytracing::object::shader::Shader;
use crate::raytracing::shading::ambient_shader::AmbientShader;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MaterialId(i32);

impl MaterialId {
    pub fn new(material_id: i32) -> Self {
        Self(material_id)
    }
}

pub enum Material {
    Ambient(Ambient),
}

impl Material {
    pub fn shader(&self) -> impl Shader {
        match self {
            Material::Ambient(material) => AmbientShader::new(material.clone()),
        }
    }
}
