use crate::core::configuration::Size;

pub struct RustTracerConfiguration {
    window_size: Size,
    default_render_size: Size,
}

impl RustTracerConfiguration {
    pub fn new(window_size: WindowSize, default_render_size: RenderSize) -> Self {
        Self {
            window_size: window_size.0,
            default_render_size: default_render_size.0,
        }
    }

    pub fn default_render_size(&self) -> &Size {
        &self.default_render_size
    }

    pub fn window_size(&self) -> &Size {
        &self.window_size
    }
}

pub struct WindowSize(Size);

impl WindowSize {
    pub fn new(size: Size) -> Self {
        Self(size)
    }
}

pub struct RenderSize(Size);

impl RenderSize {
    pub fn new(size: Size) -> Self {
        Self(size)
    }
}
