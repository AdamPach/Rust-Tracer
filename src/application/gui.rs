use crate::Size;
use crate::application::configuration::RendererConfiguration;
use crate::renderer::render::Render;
use crate::application::rendering_thread::RenderingThread;
use crate::renderer::RayTracer;
use eframe::egui::{Context, TextureHandle};
use eframe::{Frame, egui};
use std::default::Default;
use eframe::epaint::{ColorImage, ImageData};

pub struct RustTracerApplication {
    render: TextureHandle,
    rendering_thread: RenderingThread,
    window_size: Size,
}

impl RustTracerApplication {
    pub fn new(renderer_configuration: RendererConfiguration, ctx: &Context) -> Self {
        let size = renderer_configuration.size().clone();

        let renderer = RayTracer::new(renderer_configuration);

        Self {
            render: ctx.load_texture(
                "Render",
                Render::new(size.clone()),
                Default::default(),
            ),
            rendering_thread: RenderingThread::new(renderer),
            window_size: size,
        }
    }

    fn try_update_render(&mut self, ctx: &Context) {
        if let Ok(render) = self.rendering_thread.try_recv_render() {
            self.render = ctx.load_texture("Render", render, Default::default());
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

impl From<Render> for ImageData {
    fn from(render: Render) -> Self {
        let render_data = render.get_render_data();

        ColorImage::from_rgba_unmultiplied(
            [render_data.0.get_width(), render_data.0.get_height()],
            &render_data.1,
        ).into()
    }
}