use crate::Size;
use eframe::epaint::ColorImage;

pub struct Render {
    size: Size,
    pixels_rgba: Vec<u8>,
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
            pixels_rgba: Vec::with_capacity(render_size),
        }
    }

    pub fn black(size: Size) -> Self {
        let render_size = size.get_width() * size.get_height() * 4;

        Self {
            size,
            pixels_rgba: vec![0; render_size],
        }
    }

    pub fn add_pixel(&mut self, rgba: RGBA) {
        self.pixels_rgba.push(rgba.r);
        self.pixels_rgba.push(rgba.g);
        self.pixels_rgba.push(rgba.b);
        self.pixels_rgba.push(rgba.a);
    }

    pub fn to_color_image(&self) -> ColorImage {
        ColorImage::from_rgba_unmultiplied(
            [self.size.get_width(), self.size.get_height()],
            &self.pixels_rgba,
        )
    }
}
