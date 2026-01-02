use crate::core::render::RGBA;

#[derive(Clone)]
pub struct MaterialColor {
    r: R,
    g: G,
    b: B,
    a: A,
}

impl MaterialColor {
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

impl Into<RGBA> for MaterialColor {
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

    pub fn get(&self) -> f32 {
        self.0
    }
}

impl G {
    pub fn new(g: f32) -> Self {
        G(g)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

impl B {
    pub fn new(b: f32) -> Self {
        B(b)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

impl A {
    pub fn new(a: f32) -> Self {
        A(a)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}
