use crate::core::render::color::ColorRGBA;
use crate::core::render::pixels::{PixelX, PixelY};
use crate::Width;

pub struct RenderPixel {
    color: ColorRGBA,
    position: PixelPosition,
}

impl RenderPixel {
    pub fn position(&self) -> &PixelPosition {
        &self.position
    }

    pub fn color(&self) -> &ColorRGBA {
        &self.color
    }
    
    pub fn index(&self, width: &Width) -> usize
    {
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

    pub fn create_render_pixel(self, color: ColorRGBA) -> RenderPixel {
        RenderPixel {
            color,
            position: self,
        }
    }
    pub fn get_pixel_coordinates(&self) -> (PixelX, PixelY) {
        (PixelX::new(self.x as f64), PixelY::new(self.y as f64))
    }
}
