use crate::core::configuration::{RendererState, Size};

pub struct RayTracerConfiguration {
    size: Size,
}

impl RayTracerConfiguration {
    pub fn size(&self) -> &Size {
        &self.size
    }
}

impl Into<RayTracerConfiguration> for RendererState {
    fn into(self) -> RayTracerConfiguration {
        RayTracerConfiguration {
            size: self.render_size().clone(),
        }
    }
}
