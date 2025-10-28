use crate::raytracing::geometry::barycentric::Barycentric;

pub struct RayHit {
    barycentric: Barycentric,
    distance: f64,
}

impl RayHit {
    pub fn new(barycentric: Barycentric, distance: f64) -> Self {
        Self {
            barycentric,
            distance,
        }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
