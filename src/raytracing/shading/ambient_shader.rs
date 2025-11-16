use crate::raytracing::intersection::RayHit;
use crate::raytracing::object::material::ambient::Ambient;
use crate::raytracing::object::material::color::Color;
use crate::raytracing::object::shader::Shader;

pub struct AmbientShader {
    material: Ambient,
}

impl AmbientShader {
    pub fn new(material: Ambient) -> AmbientShader {
        AmbientShader { material }
    }
}

impl Shader for AmbientShader {
    fn shade(&self, _: RayHit) -> Color {
        self.material.get_color()
    }
}
