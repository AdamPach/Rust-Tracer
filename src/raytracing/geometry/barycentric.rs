use crate::raytracing::geometry::coordinates::{U, V};

pub struct Barycentric {
    u: U,
    v: V,
}

impl Barycentric {
    pub fn new(u: U, v: V) -> Self {
        Self { u, v }
    }
}
