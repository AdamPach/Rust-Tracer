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

impl Y {
    pub fn new(y: f64) -> Self {
        Self(y)
    }

    pub fn get(&self) -> f64 {
        self.0
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
