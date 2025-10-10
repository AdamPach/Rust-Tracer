use crate::application::gui::RustTracerApplication;
use eframe::{NativeOptions, egui};

pub struct RustTracer {
    options: NativeOptions,
    config: RustTracerConfiguration,
}

pub struct RustTracerConfiguration {
    pub render_width: f32,
    pub settings_width: f32,
    pub height: f32,
}

impl RustTracerConfiguration {
    fn get_window_size(&self) -> [f32; 2] {
        [self.render_width + self.settings_width, self.height]
    }
}

impl RustTracer {
    pub fn new(rust_tracer_configuration: RustTracerConfiguration) -> RustTracer {
        RustTracer {
            options: NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size(rust_tracer_configuration.get_window_size()),
                ..Default::default()
            },
            config: rust_tracer_configuration,
        }
    }

    pub fn run(self) -> eframe::Result {
        eframe::run_native(
            "Rust Tracer",
            self.options,
            Box::new(|cc| {
                Ok(Box::<RustTracerApplication>::new(
                    RustTracerApplication::new(self.config, &cc),
                ))
            }),
        )
    }
}
