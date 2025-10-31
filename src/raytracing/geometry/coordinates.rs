use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct X(f64);

#[derive(Copy, Clone)]
pub struct Y(f64);

#[derive(Copy, Clone)]
pub struct Z(f64);

impl X {
    pub fn new(x: f64) -> Self {
        Self(x)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Sub for X {
    type Output = X;

    fn sub(self, rhs: Self) -> Self::Output {
        X(self.0 - rhs.0)
    }
}

impl Y {
    pub fn new(y: f64) -> Self {
        Self(y)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Sub for Y {
    type Output = Y;

    fn sub(self, rhs: Self) -> Self::Output {
        Y(self.0 - rhs.0)
    }
}

impl Z {
    pub fn new(z: f64) -> Self {
        Self(z)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Sub for Z {
    type Output = Z;

    fn sub(self, rhs: Self) -> Self::Output {
        Z(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone)]
pub struct U(f64);

#[derive(Copy, Clone)]
pub struct V(f64);

impl U {
    pub fn new(u: f64) -> Self {
        Self(u)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl V {
    pub fn new(v: f64) -> Self {
        Self(v)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

pub struct PixelX(f64);

pub struct PixelY(f64);

impl PixelX {
    pub fn new(x: f64) -> Self {
        Self(x)
    }

    pub fn get(&self) -> f64 {
        self.0
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