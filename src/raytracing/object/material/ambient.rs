use crate::raytracing::object::material::color::Color;
use crate::raytracing::shading::Material;

#[derive(Clone)]
pub struct Ambient {
    color: Color,
}

impl Ambient {
    pub fn new(color: Color) -> Ambient {
        Ambient { color }
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Into<Material> for Ambient {
    fn into(self) -> Material {
        Material::Ambient(self)
    }
}
