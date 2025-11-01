use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;

pub struct Ray {
    origin: Point,
    direction: Vector3,
    start_distance: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3, start_distance: f64) -> Self {
        Self {
            origin,
            direction,
            start_distance,
        }
    }

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
