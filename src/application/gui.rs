use crate::Size;
use crate::application::configuration::RendererConfiguration;
use crate::application::renderer::Render;
use crate::raytracer::RayTracer;
use eframe::egui::{Context, TextureHandle};
use eframe::{Frame, egui};
use std::default::Default;
use std::sync::{Arc, RwLock};

pub struct RustTracerApplication {
    render: Option<TextureHandle>,
    renderer_configuration: RendererConfiguration,
    last_render: Arc<RwLock<Render>>,
    window_size: Size,
    ray_tracer: Arc<RwLock<RayTracer>>,
}

impl RustTracerApplication {
    pub fn new(renderer_configuration: RendererConfiguration) -> Self {
        let size = renderer_configuration.size().clone();

        let ray_tracer = Arc::new(RwLock::new(RayTracer::new(renderer_configuration.clone())));

        let last_render = Arc::new(RwLock::new(Render::black(size.clone())));

        let app = Self {
            render: None,
            renderer_configuration,
            last_render,
            window_size: size.clone(),
            ray_tracer,
        };

        app.start_rendering_thread();

        app
    }

    fn start_rendering_thread(&self) {
        let ray_tracer = Arc::clone(&self.ray_tracer);
        let last_render = Arc::clone(&self.last_render);

        std::thread::spawn(move || {
            loop {
                let renderer = ray_tracer.read().unwrap();

                let render = renderer.render_image();

                drop(renderer);

                let mut last_render_lock = last_render.write().unwrap();
                *last_render_lock = render;

                drop(last_render_lock);

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    fn try_update_render(&mut self, ctx: &Context) {
        let new_image = self.last_render.read().unwrap();

        self.render = Some(ctx.load_texture(
            "Render",
            new_image.to_color_image(),
            Default::default(),
        ));
    }
}


impl eframe::App for RustTracerApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.try_update_render(ctx);

        ctx.request_repaint_after(std::time::Duration::from_millis(16));

        egui::Window::new("Render")
            .default_width(self.window_size.get_width() as f32)
            .default_height(self.window_size.get_height() as f32)
            .show(ctx, |ui| {
                if let Some(render) = self.render.as_mut() {
                    ui.image((render.id(), ui.available_size()));
                }
            });

        egui::Window::new("Settings")
            .default_width(200.0)
            .resizable(false)
            .show(ctx, |_ui| {});
    }
}
