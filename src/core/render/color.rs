use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    r: R,
    g: G,
    b: B,
    a: A,
}

impl Color {
    pub fn new(r: R, g: G, b: B, a: A) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self {
            r: R::new(0.0),
            g: G::new(0.0),
            b: B::new(0.0),
            a: A::new(1.0),
        }
    }

    pub fn r(&self) -> R {
        self.r.clone()
    }

    pub fn g(&self) -> G {
        self.g.clone()
    }

    pub fn b(&self) -> B {
        self.b.clone()
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: R::new(self.r.0 + rhs.r.0),
            g: G::new(self.g.0 + rhs.g.0),
            b: B::new(self.b.0 + rhs.b.0),
            a: A::new(self.a.0 + rhs.a.0),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl Into<ColorU8> for Color {
    fn into(self) -> ColorU8 {
        ColorU8 {
            r: (self.r.0.clamp(0.0, 1.0) * 255.0) as u8,
            g: (self.g.0.clamp(0.0, 1.0) * 255.0) as u8,
            b: (self.b.0.clamp(0.0, 1.0) * 255.0) as u8,
            a: (self.a.0.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct R(f64);

impl R {
    pub fn new(r: f64) -> Self {
        R(r)
    }
}

impl Mul<R> for R {
    type Output = R;

    fn mul(self, rhs: R) -> Self::Output {
        R(self.0 * rhs.0)
    }
}

impl Mul<f64> for R {
    type Output = R;

    fn mul(self, rhs: f64) -> Self::Output {
        R(self.0 * rhs)
    }
}

impl Div<f64> for R {
    type Output = R;

    fn div(self, rhs: f64) -> Self::Output {
        R(self.0 / rhs)
    }
}

#[derive(Clone, Copy)]
pub struct G(f64);

impl G {
    pub fn new(g: f64) -> Self {
        G(g)
    }
}

impl Mul<G> for G {
    type Output = G;

    fn mul(self, rhs: G) -> Self::Output {
        G(self.0 * rhs.0)
    }
}

impl Mul<f64> for G {
    type Output = G;

    fn mul(self, rhs: f64) -> Self::Output {
        G(self.0 * rhs)
    }
}

impl Div<f64> for G {
    type Output = G;

    fn div(self, rhs: f64) -> Self::Output {
        G(self.0 / rhs)
    }
}

#[derive(Clone, Copy)]
pub struct B(f64);

impl B {
    pub fn new(b: f64) -> Self {
        B(b)
    }
}

impl Mul<B> for B {
    type Output = B;

    fn mul(self, rhs: B) -> Self::Output {
        B(self.0 * rhs.0)
    }
}

impl Mul<f64> for B {
    type Output = B;

    fn mul(self, rhs: f64) -> Self::Output {
        B(self.0 * rhs)
    }
}

impl Div<f64> for B {
    type Output = B;

    fn div(self, rhs: f64) -> Self::Output {
        B(self.0 / rhs)
    }
}

#[derive(Clone, Copy)]
pub struct A(f64);

impl A {
    pub fn new(a: f64) -> Self {
        A(a)
    }
}

impl Mul<A> for A {
    type Output = A;

    fn mul(self, rhs: A) -> Self::Output {
        A(self.0 * rhs.0)
    }
}

impl Mul<f64> for A {
    type Output = A;

    fn mul(self, rhs: f64) -> Self::Output {
        A(self.0 * rhs)
    }
}

impl Div<f64> for A {
    type Output = A;

    fn div(self, rhs: f64) -> Self::Output {
        A(self.0 / rhs)
    }
}

pub struct ColorU8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ColorU8 {
    pub fn get(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}
