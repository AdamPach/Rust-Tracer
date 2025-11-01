pub struct ColorRGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ColorRGBA {
    pub fn new(r: R, g: G, b: B, a: A) -> Self {
        Self {
            r: r.0,
            g: g.0,
            b: b.0,
            a: a.0,
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

pub struct R(u8);
pub struct G(u8);
pub struct B(u8);
pub struct A(u8);

impl R {
    pub fn new(r: u8) -> Self {
        R(r)
    }
}

impl G {
    pub fn new(g: u8) -> Self {
        G(g)
    }
}

impl B {
    pub fn new(b: u8) -> Self {
        B(b)
    }
}

impl A {
    pub fn new(a: u8) -> Self {
        A(a)
    }
}
