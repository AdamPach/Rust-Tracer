use crate::core::render::{PixelX, PixelY, RGBA};
use crate::{Size, Width};

pub enum RenderState {
    InProgress,
    Completed,
}

pub struct Render {
    size: Size,
    pixels_rgba: Vec<u8>,
    x_pos: usize,
    y_pos: usize,
    missing_pixels: u32,
}

impl Render {
    pub fn new(size: Size) -> Self {
        let render_size = size.get_width() * size.get_height();

        Self {
            size,
            pixels_rgba: vec![0; render_size * 4],
            x_pos: 0,
            y_pos: 0,
            missing_pixels: render_size as u32,
        }
    }

    pub fn add_pixel(&mut self, pixel: RenderPixel) -> RenderState {
        let index = 4 * pixel.index(&self.size.get_width());

        let color = pixel.color();

        self.pixels_rgba[index] = color.r();
        self.pixels_rgba[index + 1] = color.g();
        self.pixels_rgba[index + 2] = color.b();
        self.pixels_rgba[index + 3] = color.a();

        self.missing_pixels -= 1;

        match self.missing_pixels {
            0 => RenderState::Completed,
            _ => RenderState::InProgress,
        }
    }

    pub fn get_render_data(self) -> (Size, Vec<u8>) {
        (self.size, self.pixels_rgba)
    }
}

impl Iterator for Render {
    type Item = PixelPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_pos >= self.size.get_height().get() {
            return None;
        }

        let current_position = PixelPosition {
            x: self.x_pos,
            y: self.y_pos,
        };

        self.x_pos += 1;

        if self.x_pos >= self.size.get_width().get() {
            self.x_pos = 0;
            self.y_pos += 1;
        }

        Some(current_position)
    }
}

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
