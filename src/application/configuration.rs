use crate::WindowSize;

pub struct RendererConfiguration{
    size: WindowSize,
}

impl RendererConfiguration {
    pub fn new(size: WindowSize) -> Self {
        RendererConfiguration { size }
    }
    
    pub fn size(&self) -> &WindowSize {
        &self.size
    }
}

pub struct RustTracerConfiguration{
    window_size: WindowSize,
    renderer_configuration: RendererConfiguration,
}

impl RustTracerConfiguration{
    pub fn new(window_size:WindowSize, renderer_configuration: RendererConfiguration) -> RustTracerConfiguration{
        RustTracerConfiguration{
            window_size,
            renderer_configuration,
        }
    }
    
    pub fn size(&self)->&WindowSize{
        &self.window_size
    }
}

impl From<RustTracerConfiguration> for RendererConfiguration{
    fn from(value: RustTracerConfiguration) -> Self {
        value.renderer_configuration
    }
}