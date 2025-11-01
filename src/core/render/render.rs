use crate::Size;
use crate::core::render::render_pixel::{PixelPosition, RenderPixel};

pub struct Render {
    size: Size,
    pixels_rgba: Vec<u8>,
    x_pos: usize,
    y_pos: usize,
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
        let index = 4 * pixel.index(&self.size.get_width());

        self.pixels_rgba[index] = pixel.color().r();
        self.pixels_rgba[index + 1] = pixel.color().g();
        self.pixels_rgba[index + 2] = pixel.color().b();
        self.pixels_rgba[index + 3] = pixel.color().a();
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

        let current_position = PixelPosition::new(
            self.x_pos,
            self.y_pos,
        );

        self.x_pos += 1;

        if self.x_pos >= self.size.get_width().get() {
            self.x_pos = 0;
            self.y_pos += 1;
        }

        Some(current_position)
    }
}
