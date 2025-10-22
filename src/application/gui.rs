use crate::Size;
use crate::application::configuration::RendererConfiguration;
use crate::application::renderer::Render;
use crate::raytracer::RayTracer;
use eframe::egui::{Context, TextureHandle};
use eframe::{Frame, egui};
use std::default::Default;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};

pub struct RustTracerApplication {
    render: TextureHandle,
    render_receiver: Receiver<Render>,
    window_size: Size,
    ray_tracer: Arc<Mutex<RayTracer>>,
}

impl RustTracerApplication {
    pub fn new(renderer_configuration: RendererConfiguration, ctx: &Context) -> Self {
        let size = renderer_configuration.size().clone();

        let ray_tracer = Arc::new(Mutex::new(RayTracer::new(renderer_configuration)));

        let (sender, receiver) = mpsc::channel();

        let app = Self {
            render: ctx.load_texture(
                "Render",
                Render::black(size.clone()).to_color_image(),
                Default::default(),
            ),
            render_receiver: receiver,
            window_size: size,
            ray_tracer,
        };

        app.start_rendering_thread(sender);

        app
    }

    fn start_rendering_thread(&self, render_sender: Sender<Render>) {
        let ray_tracer = Arc::clone(&self.ray_tracer);

        std::thread::spawn(move || {
            loop {
                let renderer = ray_tracer.lock().unwrap();

                let render = renderer.render_image();

                drop(renderer);

                render_sender.send(render).unwrap();

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    fn try_update_render(&mut self, ctx: &Context) {
        if let Ok(render) = self.render_receiver.try_recv() {
            self.render = ctx.load_texture("Render", render.to_color_image(), Default::default());
        }
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
                ui.image((self.render.id(), ui.available_size()));
            });

        egui::Window::new("Settings")
            .default_width(200.0)
            .resizable(false)
            .show(ctx, |_ui| {});
    }
}
