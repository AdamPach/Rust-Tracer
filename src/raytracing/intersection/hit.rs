use crate::raytracing::geometry::barycentric::Barycentric;
use crate::raytracing::intersection::ray_hit::RayHit;

pub struct Hit {
    barycentric: Barycentric,
    distance: f64,
}

impl Hit {
    pub fn new(barycentric: Barycentric, distance: f64) -> Self {
        Self {
            barycentric,
            distance,
        }
    }

    pub fn into_ray_hit(self) -> RayHit {
        RayHit::new(self.barycentric, self.distance)
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
