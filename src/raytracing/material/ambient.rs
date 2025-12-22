use crate::raytracing::material::MaterialType;
use crate::raytracing::material::color::MaterialColor;

#[derive(Clone)]
pub struct AmbientMaterial {
    color: MaterialColor,
}

impl AmbientMaterial {
    pub fn get_color(&self) -> MaterialColor {
        self.color.clone()
    }
}

pub struct AmbientMaterialBuilder(MaterialColor);

impl AmbientMaterialBuilder {
    pub fn new(color: MaterialColor) -> Self {
        AmbientMaterialBuilder(color)
    }
}

impl Into<MaterialType> for AmbientMaterialBuilder {
    fn into(self) -> MaterialType {
        MaterialType::Ambient(AmbientMaterial { color: self.0 })
    }
}
