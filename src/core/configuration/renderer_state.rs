use crate::core::configuration::{RustTracerConfiguration, Size};

#[derive(Clone)]
pub struct RendererState {
    render_size: Size,
}

impl RendererState {
    pub fn render_size(&self) -> &Size {
        &self.render_size
    }
}

impl Into<RendererState> for RustTracerConfiguration {
    fn into(self) -> RendererState {
        RendererState {
            render_size: self.default_render_size().clone(),
        }
    }
}
