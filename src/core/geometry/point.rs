use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::vector::Vector3;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Point {
    x: X,
    y: Y,
    z: Z,
}

impl Point {
    pub fn new(x: X, y: Y, z: Z) -> Point {
        Point { x, y, z }
    }
}

impl Sub for Point {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(
            X::new(self.x.get() - rhs.x.get()),
            Y::new(self.y.get() - rhs.y.get()),
            Z::new(self.z.get() - rhs.z.get()),
        )
    }
}
