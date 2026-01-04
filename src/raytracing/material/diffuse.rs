use crate::core::render::color::Color;
use crate::raytracing::material::MaterialType;

pub struct DiffuseMaterial {
    albedo: Color,
}

impl DiffuseMaterial {
    pub fn albedo(&self) -> Color {
        self.albedo.clone()
    }
}

pub struct DiffuseMaterialBuilder {
    albedo: Color,
}

impl DiffuseMaterialBuilder {
    pub fn new(albedo: Color) -> Self {
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
