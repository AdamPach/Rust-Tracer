use crate::raytracing::material::MaterialType;
use crate::raytracing::material::color::Color;

pub struct EmissiveMaterial {
    emission: Color,
}

impl EmissiveMaterial {
    pub fn emission(&self) -> Color {
        self.emission.clone()
    }
}

pub struct EmissiveMaterialBuilder {
    emission: Color,
}

impl EmissiveMaterialBuilder {
    pub fn new(emission: Color) -> Self {
        Self { emission }
    }
}

impl Into<MaterialType> for EmissiveMaterialBuilder {
    fn into(self) -> MaterialType {
        MaterialType::Emissive(EmissiveMaterial {
            emission: self.emission,
        })
    }
}
