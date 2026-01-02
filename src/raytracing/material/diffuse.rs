use crate::raytracing::material::MaterialType;
use crate::raytracing::material::color::MaterialColor;

pub struct DiffuseMaterial {
    albedo: MaterialColor,
}

impl DiffuseMaterial {
    pub fn albedo(&self) -> MaterialColor {
        self.albedo.clone()
    }
}

pub struct DiffuseMaterialBuilder {
    albedo: MaterialColor,
}

impl DiffuseMaterialBuilder {
    pub fn new(albedo: MaterialColor) -> Self {
        Self { albedo }
    }
}

impl Into<MaterialType> for DiffuseMaterialBuilder {
    fn into(self) -> MaterialType {
        MaterialType::Diffuse(DiffuseMaterial {
            albedo: self.albedo,
        })
    }
}
