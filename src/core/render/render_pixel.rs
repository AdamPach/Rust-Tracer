use crate::Width;
use crate::core::render::pixels::{PixelX, PixelY};
use crate::core::render::rgba::RGBA;

pub struct RenderPixel {
    color: RGBA,
    position: PixelPosition,
}

impl RenderPixel {
    pub fn color(self) -> RGBA {
        self.color
    }

    pub fn index(&self, width: &Width) -> usize {
        self.position.x + self.position.y * width.get()
    }
}

pub struct PixelPosition {
    x: usize,
    y: usize,
}

impl PixelPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn create_render_pixel(self, color: impl Into<RGBA>) -> RenderPixel {
        RenderPixel {
            color: color.into(),
            position: self,
        }
    }
    pub fn get_pixel_coordinates(&self) -> (PixelX, PixelY) {
        (PixelX::new(self.x as f64), PixelY::new(self.y as f64))
    }
}
