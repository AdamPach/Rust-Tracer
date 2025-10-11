use crate::Size;
use crate::application::configuration::RendererConfiguration;
use crate::application::renderer::Render;
use crate::raytracer::RayTracer;
use eframe::egui::{Context, TextureHandle};
use eframe::{Frame, egui};
use std::default::Default;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock, mpsc};

pub struct RustTracerApplication {
    render: Option<TextureHandle>,
    renderer_configuration: Arc<RwLock<RendererConfiguration>>,
    render_receiver: Receiver<Render>,
    window_size: Size,
    ray_tracer: RayTracer,
}

impl RustTracerApplication {
    pub fn new(renderer_configuration: Arc<RwLock<RendererConfiguration>>) -> Self {
        let (tx, rx) = mpsc::channel();

        let conf = renderer_configuration.read().unwrap();

        let size = conf.size().clone();

        drop(conf);

        let mut ray_tracer = RayTracer::new(renderer_configuration.clone());

        ray_tracer.start(tx);

        Self {
            render: None,
            renderer_configuration,
            render_receiver: rx,
            window_size: size,
            ray_tracer,
        }
    }

    fn try_update_render(&mut self, ctx: &egui::Context) {
        let new_image = self.render_receiver.try_recv().ok();

        match new_image {
            Some(new_image) => {
                self.render = Some(ctx.load_texture(
                    "Render",
                    new_image.to_color_image(),
                    Default::default(),
                ));
            }
            None => return,
        }
    }
}

impl eframe::App for RustTracerApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.try_update_render(ctx);

        egui::Window::new("Render")
            .default_width(self.window_size.get_height() as f32)
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
