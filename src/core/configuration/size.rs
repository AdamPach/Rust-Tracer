use std::ops::{Div, Mul};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Width(usize);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Height(usize);

#[derive(Clone, Eq, PartialEq)]
pub struct Size {
    height: Height,
    width: Width,
}

impl Size {
    pub fn new(width: Width, height: Height) -> Self {
        Size { height, width }
    }

    pub fn get_size(&self) -> [f32; 2] {
        [self.width.0 as f32, self.height.0 as f32]
    }

    pub fn get_width(&self) -> Width {
        self.width.clone()
    }

    pub fn get_height(&self) -> Height {
        self.height.clone()
    }
}

impl Width {
    pub fn new(width: usize) -> Self {
        Width(width)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Height {
    pub fn new(height: usize) -> Self {
        Height(height)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Into<f32> for Width {
    fn into(self) -> f32 {
        self.0 as f32
    }
}

impl Into<f32> for Height {
    fn into(self) -> f32 {
        self.0 as f32
    }
}

impl Mul<Width> for Height {
    type Output = usize;

    fn mul(self, rhs: Width) -> Self::Output {
        self.0 * rhs.0
    }
}

impl Div<f64> for Width {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.0 as f64 / rhs
    }
}

impl Div<f64> for Height {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.0 as f64 / rhs
    }
}

impl Mul<Height> for Width {
    type Output = usize;

    fn mul(self, rhs: Height) -> Self::Output {
        self.0 * rhs.0
    }
}
