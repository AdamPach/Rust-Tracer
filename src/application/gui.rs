use eframe::egui::{ColorImage, Context, TextureHandle};
use eframe::{Frame, egui};
use std::default::Default;
use crate::application::configuration::RendererConfiguration;

pub struct RustTracerApplication {
    render: Option<TextureHandle>,
    renderer_configuration: RendererConfiguration,
}

impl RustTracerApplication {
    pub fn new(cc: &eframe::CreationContext, renderer_configuration: RendererConfiguration) -> Self {
        let pixels = [0; 400 * 400 * 3];

        let color_image = ColorImage::from_rgb([400, 400], &pixels);

        let render = cc
            .egui_ctx
            .load_texture("Render", color_image, Default::default());

        Self {
            render: Some(render),
            renderer_configuration
        }
    }
}

impl eframe::App for RustTracerApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::Window::new("Render")
            .default_width(self.renderer_configuration.size().get_width())
            .default_height(self.renderer_configuration.size().get_height())
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
