use crate::raytracing::material::MaterialType;
use crate::raytracing::material::color::MaterialColor;

pub struct EmissiveMaterial {
    emission: MaterialColor,
}

impl EmissiveMaterial {
    pub fn emission(&self) -> MaterialColor {
        self.emission.clone()
    }
}

pub struct EmissiveMaterialBuilder {
    emission: MaterialColor,
}

impl EmissiveMaterialBuilder {
    pub fn new(emission: MaterialColor) -> Self {
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
