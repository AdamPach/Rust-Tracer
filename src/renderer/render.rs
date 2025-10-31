use crate::Size;
use crate::raytracing::geometry::coordinates::{PixelX, PixelY};

pub struct Render {
    size: Size,
    pixels_rgba: Vec<u8>,
    x_pos: usize,
    y_pos: usize,
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Render {
    pub fn new(size: Size) -> Self {
        let render_size = size.get_width() * size.get_height() * 4;

        Self {
            size,
            pixels_rgba: vec![0; render_size],
            x_pos: 0,
            y_pos: 0,
        }
    }

    pub fn add_pixel(&mut self, pixel: RenderPixel) {
        let index = 4 * (pixel.position.x + self.size.get_width() * pixel.position.y);

        self.pixels_rgba[index] = pixel.rgba.r;
        self.pixels_rgba[index + 1] = pixel.rgba.g;
        self.pixels_rgba[index + 2] = pixel.rgba.b;
        self.pixels_rgba[index + 3] = pixel.rgba.a;
    }

    pub fn get_render_data(self) -> (Size, Vec<u8>) {
        (self.size, self.pixels_rgba)
    }
}

impl Iterator for Render{
    type Item = PixelPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_pos >= self.size.get_height() {
            return None;
        }

        let current_position = PixelPosition {
            x: self.x_pos,
            y: self.y_pos,
        };

        self.x_pos += 1;

        if self.x_pos >= self.size.get_width() {
            self.x_pos = 0;
            self.y_pos += 1;
        }

        Some(current_position)
    }
}

pub struct RenderPixel {
    rgba: RGBA,
    position: PixelPosition,
}

pub struct PixelPosition {
    x: usize,
    y: usize,
}

impl PixelPosition {
    pub fn create_render_pixel(self, rgba: RGBA) -> RenderPixel {
        RenderPixel {
            rgba,
            position: self,
        }
    }
    pub fn get_pixel_coordinates(&self) -> (PixelX, PixelY) {
        (PixelX::new(self.x as f64), PixelY::new(self.y as f64))
    }
}