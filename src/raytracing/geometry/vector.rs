use crate::raytracing::geometry::coordinates::{X, Y, Z};

pub struct Vector3 {
    x: X,
    y: Y,
    z: Z,
}

impl Vector3 {
    pub fn new(x: X, y: Y, z: Z) -> Self {
        Self { x, y, z }
    }

    pub fn norm(self) -> Self {
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
}
