use crate::core::render::color::{Color, ColorU8};
use crate::core::render::{PixelX, PixelY};
use crate::{Size, Width};

pub enum RenderState {
    InProgress,
    Completed,
}

pub struct RenderAccumulator {
    size: Size,
    pixels: Vec<Color>,
    count: u32,
}

impl RenderAccumulator {
    pub fn new(size: Size) -> Self {
        let render_size = size.get_width() * size.get_height();

        Self {
            size,
            pixels: vec![Color::black(); render_size * 4],
            count: 0,
        }
    }

    pub fn accumulated_render(&mut self) -> AccumulatedRender<'_> {
        let render_size = self.size.get_width() * self.size.get_height();
        let size = self.size.clone();

        AccumulatedRender {
            accumulator: self,
            render: Render::new(size),
            missing_pixels: render_size as u32,
        }
    }

    fn accumulate_pixel(&mut self, pixel_position: &PixelPosition, color: Color) -> Color {
        let index = pixel_position.index(&self.size.get_width());

        let accumulated_color = &self.pixels[index];

        let new_color =
            (accumulated_color.clone() * self.count as f64 + color) / (self.count as f64 + 1.0);

        self.pixels[index] = new_color;

        new_color
    }
}

pub struct AccumulatedRender<'a> {
    accumulator: &'a mut RenderAccumulator,
    render: Render,
    missing_pixels: u32,
}

impl AccumulatedRender<'_> {
    pub fn iterator(&self) -> RenderIterator {
        RenderIterator {
            size: self.accumulator.size.clone(),
            x_pos: 0,
            y_pos: 0,
        }
    }

    pub fn add_pixel(&mut self, pixel: RenderPixel) -> RenderState {
        let position = pixel.position;

        let color = self.accumulator.accumulate_pixel(&position, pixel.color);

        self.render.add_pixel(&position, color);

        self.missing_pixels -= 1;

        match self.missing_pixels {
            0 => RenderState::Completed,
            _ => RenderState::InProgress,
        }
    }

    pub fn get_render(self) -> Render {
        self.accumulator.count += 1;
        self.render
    }
}

pub struct RenderIterator {
    size: Size,
    x_pos: usize,
    y_pos: usize,
}

impl Iterator for RenderIterator {
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

pub struct Render {
    size: Size,
    pixels_rgba: Vec<u8>,
}

impl Render {
    pub fn new(size: Size) -> Self {
        let render_size = size.get_width() * size.get_height();

        Self {
            size,
            pixels_rgba: vec![0; render_size * 4],
        }
    }

    fn add_pixel<T: Into<ColorU8>>(&mut self, pixel_position: &PixelPosition, pixel: T) {
        let index = 4 * pixel_position.index(&self.size.get_width());

        let (r, g, b, a) = pixel.into().get();

        self.pixels_rgba[index] = r;
        self.pixels_rgba[index + 1] = g;
        self.pixels_rgba[index + 2] = b;
        self.pixels_rgba[index + 3] = a;
    }

    pub fn get_render_data(self) -> (Size, Vec<u8>) {
        (self.size, self.pixels_rgba)
    }
}

pub struct RenderPixel {
    color: Color,
    position: PixelPosition,
}

pub struct PixelPosition {
    x: usize,
    y: usize,
}

impl PixelPosition {
    pub fn create_render_pixel(self, color: Color) -> RenderPixel {
        RenderPixel {
            color,
            position: self,
        }
    }
    pub fn get_pixel_coordinates(&self) -> (PixelX, PixelY) {
        (PixelX::new(self.x as f64), PixelY::new(self.y as f64))
    }

    fn index(&self, width: &Width) -> usize {
        self.x + self.y * width.get()
    }
}
