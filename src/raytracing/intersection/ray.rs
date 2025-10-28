use crate::raytracing::geometry::point::Point;
use crate::raytracing::geometry::vector::Vector3;

pub struct Ray {
    origin: Point,
    direction: Vector3,
    start_distance: f64,
}

impl Ray {
    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn start_distance(&self) -> f64 {
        self.start_distance
    }
}
