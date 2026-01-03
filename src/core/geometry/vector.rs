use crate::core::geometry::coordinates::{X, Y, Z};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    x: X,
    y: Y,
    z: Z,
}

impl Vector3 {
    pub fn new(x: X, y: Y, z: Z) -> Self {
        Self { x, y, z }
    }

    pub fn norm(&self) -> Self {
        let x = self.x.get();
        let y = self.y.get();
        let z = self.z.get();

        let length = (x * x + y * y + z * z).sqrt();

        Self {
            x: X::new(x / length),
            y: Y::new(y / length),
            z: Z::new(z / length),
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x.get() * other.x.get() + self.y.get() * other.y.get() + self.z.get() * other.z.get()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: X::new(self.y.get() * other.z.get() - self.z.get() * other.y.get()),
            y: Y::new(self.z.get() * other.x.get() - self.x.get() * other.z.get()),
            z: Z::new(self.x.get() * other.y.get() - self.y.get() * other.x.get()),
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

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: X::new(self.x.get() * rhs),
            y: Y::new(self.y.get() * rhs),
            z: Z::new(self.z.get() * rhs),
        }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: X::new(0.0),
            y: Y::new(0.0),
            z: Z::new(0.0),
        }
    }
}
