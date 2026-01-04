use crate::core::render::color::Color;
use crate::raytracing::material::MaterialType;

pub struct AmbientMaterial {
    color: Color,
}

impl AmbientMaterial {
    pub fn get_color(&self) -> Color {
        self.color.clone()
    }
}

pub struct AmbientMaterialBuilder(Color);

impl AmbientMaterialBuilder {
    pub fn new(color: Color) -> Self {
        AmbientMaterialBuilder(color)
    }
}

impl Into<MaterialType> for AmbientMaterialBuilder {
    fn into(self) -> MaterialType {
        MaterialType::Ambient(AmbientMaterial { color: self.0 })
    }
}
