use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::vector::Vector3;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: X,
    y: Y,
    z: Z,
}

impl Point {
    pub fn new(x: X, y: Y, z: Z) -> Point {
        Point { x, y, z }
    }

    pub fn default() -> Point {
        Point {
            x: X::new(0.0),
            y: Y::new(0.0),
            z: Z::new(0.0),
        }
    }

    pub fn x(&self) -> X {
        self.x
    }

    pub fn y(&self) -> Y {
        self.y
    }

    pub fn z(&self) -> Z {
        self.z
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
