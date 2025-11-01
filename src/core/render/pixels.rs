use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct PixelX(f64);

#[derive(Copy, Clone)]
pub struct PixelY(f64);

impl PixelX {
    pub fn new(x: f64) -> Self {
        Self(x)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Sub<f64> for PixelX {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        self.0 - rhs
    }
}

impl PixelY {
    pub fn new(y: f64) -> Self {
        Self(y)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Sub<f64> for PixelY {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        self.0 - rhs
    }
}
