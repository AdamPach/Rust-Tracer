use crate::RendererConfiguration;
use crate::application::renderer::{RGBA, Render};
use rand::Rng;

pub struct RayTracer {
    configuration: RendererConfiguration,
}

impl RayTracer {
    pub fn new(configuration: RendererConfiguration) -> Self {
        Self { configuration }
    }

    pub fn render_image(&self) -> Render {
        let size = self.configuration.size().clone();

        let mut render = Render::new(size.clone());

        let mut rng = rand::rng();

        let r = rng.random::<u8>();
        let g = rng.random::<u8>();
        let b = rng.random::<u8>();

        for _h in 0..size.get_height() {
            for _w in 0..size.get_width() {
                let rgba = RGBA { r, g, b, a: 255 };

                render.add_pixel(rgba);
            }
        }

        render
    }
}
