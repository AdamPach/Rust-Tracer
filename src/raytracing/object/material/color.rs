use crate::core::render::RGBA;

#[derive(Clone)]
pub struct Color {
    r: R,
    g: G,
    b: B,
    a: A,
}

impl Color {
    pub fn new(r: R, g: G, b: B, a: A) -> Color {
        Color { r, g, b, a }
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
#[derive(Clone)]
pub struct G(f32);
#[derive(Clone)]
pub struct B(f32);
#[derive(Clone)]
pub struct A(f32);

impl R {
    pub fn new(r: f32) -> Self {
        R(r)
    }
}

impl G {
    pub fn new(g: f32) -> Self {
        G(g)
    }
}

impl B {
    pub fn new(b: f32) -> Self {
        B(b)
    }
}

impl A {
    pub fn new(a: f32) -> Self {
        A(a)
    }
}
