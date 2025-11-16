use crate::core::geometry::barycentric::Barycentric;
use crate::raytracing::shading::{MaterialId};

pub struct RayHit {
    barycentric: Barycentric,
    distance: f64,
    material_id: MaterialId,
}

impl RayHit {
    pub fn new(barycentric: Barycentric, material_id: MaterialId, distance: f64) -> Self {
        Self {
            barycentric,
            distance,
            material_id,
        }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }
}
