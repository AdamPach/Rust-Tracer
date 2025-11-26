use crate::raytracing::RayHit;
use crate::raytracing::object::material::ambient::AmbientMaterial;
use crate::raytracing::object::material::color::Color;
use crate::renderer::raytracer::shading::shaders::Shader;

pub struct AmbientShader {
    material: AmbientMaterial,
}

impl AmbientShader {
    pub fn new(material: AmbientMaterial) -> AmbientShader {
        AmbientShader { material }
    }
}

impl Shader for AmbientShader {
    fn shade(&self, _: RayHit) -> Color {
        self.material.get_color()
    }
}
