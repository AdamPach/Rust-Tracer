#[derive(Eq, PartialEq, Clone)]
pub struct RendererConfiguration {
    size: Size,
}

impl RendererConfiguration {
    pub fn new(size: Size) -> Self {
        RendererConfiguration { size }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

pub struct RustTracerConfiguration {
    window_size: Size,
    renderer_configuration: RendererConfiguration,
}

impl RustTracerConfiguration {
    pub fn new(
        window_size: Size,
        renderer_configuration: RendererConfiguration,
    ) -> RustTracerConfiguration {
        RustTracerConfiguration {
            window_size,
            renderer_configuration,
        }
    }

    pub fn size(&self) -> &Size {
        &self.window_size
    }
}

impl From<RustTracerConfiguration> for RendererConfiguration {
    fn from(value: RustTracerConfiguration) -> Self {
        value.renderer_configuration
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Width(pub usize);

#[derive(Clone, Eq, PartialEq)]
pub struct Height(pub usize);

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

    pub fn get_width(&self) -> usize {
        self.width.0
    }

    pub fn get_height(&self) -> usize {
        self.height.0
    }
}
