use std::default::Default;
use crate::RustTracerConfiguration;
use eframe::egui::{ColorImage, Context, TextureHandle};
use eframe::{Frame, egui};
pub struct RustTracerApplication {
    configuration: RustTracerConfiguration,
    render: Option<TextureHandle>,
}

impl RustTracerApplication {
    pub fn new(configuration: RustTracerConfiguration, cc:&eframe::CreationContext) -> Self {
        let pixels = [0; 400 * 400 * 3];

        let color_image = ColorImage::from_rgb([400, 400], &pixels);

        let render = cc.egui_ctx.load_texture(
            "Render",
            color_image,
            Default::default(),
        );

        Self {
            configuration,
            render: Some(render),
        }
    }
}

impl eframe::App for RustTracerApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("image_panel")
            .default_width(self.configuration.render_width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Render Output");
                ui.separator();

                if let Some(render) = self.render.as_mut() {
                    ui.image((render.id(), ui.available_size()));
                }
            });

        egui::SidePanel::right("settings_panel")
            .default_width(self.configuration.settings_width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Settings");
                ui.separator();
            });
    }
}
