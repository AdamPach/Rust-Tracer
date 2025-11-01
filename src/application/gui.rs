use crate::application::rendering_thread::RenderingThread;
use crate::core::configuration::RendererState;
use crate::core::render::Render;
use crate::renderer::raytracer::RayTracer;
use eframe::egui::{Context, TextureHandle};
use eframe::epaint::{ColorImage, ImageData};
use eframe::{Frame, egui};
use std::default::Default;

pub struct RustTracerApplication {
    render: TextureHandle,
    rendering_thread: RenderingThread,
    state: RendererState,
}

impl RustTracerApplication {
    pub fn new(into_state: impl Into<RendererState>, ctx: &Context) -> Self {
        let state: RendererState = into_state.into();

        let renderer = RayTracer::new(state.clone());

        Self {
            render: ctx.load_texture(
                "Render",
                Render::new(state.render_size().clone()),
                Default::default(),
            ),
            rendering_thread: RenderingThread::new(renderer),
            state,
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
            .default_width(self.state.render_size().get_width().into())
            .default_height(self.state.render_size().get_height().into())
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
            [
                render_data.0.get_width().get(),
                render_data.0.get_height().get(),
            ],
            &render_data.1,
        )
        .into()
    }
}
