use crate::core::render::RGBA;
use std::ops::{Div, Mul};

#[derive(Clone)]
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

    pub fn r(&self) -> R {
        self.r.clone()
    }

    pub fn g(&self) -> G {
        self.g.clone()
    }

    pub fn b(&self) -> B {
        self.b.clone()
    }

    pub fn a(&self) -> A {
        self.a.clone()
    }
}

impl Into<RGBA> for Color {
    fn into(self) -> RGBA {
        RGBA::new(
            (self.r.0.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.0.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.0.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.0.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

#[derive(Clone)]
pub struct R(f32);

impl R {
    pub fn new(r: f32) -> Self {
        R(r)
    }
}

impl Mul<R> for R {
    type Output = R;

    fn mul(self, rhs: R) -> Self::Output {
        R(self.0 * rhs.0)
    }
}

impl Mul<f32> for R {
    type Output = R;

    fn mul(self, rhs: f32) -> Self::Output {
        R(self.0 * rhs)
    }
}

impl Div<f32> for R {
    type Output = R;

    fn div(self, rhs: f32) -> Self::Output {
        R(self.0 / rhs)
    }
}

#[derive(Clone)]
pub struct G(f32);

impl G {
    pub fn new(g: f32) -> Self {
        G(g)
    }
}

impl Mul<G> for G {
    type Output = G;

    fn mul(self, rhs: G) -> Self::Output {
        G(self.0 * rhs.0)
    }
}

impl Mul<f32> for G {
    type Output = G;

    fn mul(self, rhs: f32) -> Self::Output {
        G(self.0 * rhs)
    }
}

impl Div<f32> for G {
    type Output = G;

    fn div(self, rhs: f32) -> Self::Output {
        G(self.0 / rhs)
    }
}

#[derive(Clone)]
pub struct B(f32);

impl B {
    pub fn new(b: f32) -> Self {
        B(b)
    }
}

impl Mul<B> for B {
    type Output = B;

    fn mul(self, rhs: B) -> Self::Output {
        B(self.0 * rhs.0)
    }
}

impl Mul<f32> for B {
    type Output = B;

    fn mul(self, rhs: f32) -> Self::Output {
        B(self.0 * rhs)
    }
}

impl Div<f32> for B {
    type Output = B;

    fn div(self, rhs: f32) -> Self::Output {
        B(self.0 / rhs)
    }
}

#[derive(Clone)]
pub struct A(f32);

impl A {
    pub fn new(a: f32) -> Self {
        A(a)
    }
}

impl Mul<A> for A {
    type Output = A;

    fn mul(self, rhs: A) -> Self::Output {
        A(self.0 * rhs.0)
    }
}

impl Mul<f32> for A {
    type Output = A;

    fn mul(self, rhs: f32) -> Self::Output {
        A(self.0 * rhs)
    }
}

impl Div<f32> for A {
    type Output = A;

    fn div(self, rhs: f32) -> Self::Output {
        A(self.0 / rhs)
    }
}
