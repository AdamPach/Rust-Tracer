use crate::core::geometry::barycentric::Barycentric;
use crate::raytracing::object::material::material_type::MaterialTypeId;

pub struct RayHit {
    barycentric: Barycentric,
    distance: f64,
    material_id: MaterialTypeId,
}

impl RayHit {
    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn material_id(&self) -> MaterialTypeId {
        self.material_id
    }
}

pub struct Hit {
    barycentric: Barycentric,
    distance: f64,
    material_id: MaterialTypeId,
}

impl Hit {
    pub fn new(barycentric: Barycentric, material_id: MaterialTypeId, distance: f64) -> Self {
        Self {
            barycentric,
            distance,
            material_id,
        }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn ray_hit(self) -> RayHit {
        RayHit {
            barycentric: self.barycentric,
            distance: self.distance,
            material_id: self.material_id,
        }
    }
}
