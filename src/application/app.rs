use crate::application::gui::RustTracerApplication;
use crate::core::configuration::RustTracerConfiguration;
use eframe::{NativeOptions, egui};

pub struct RustTracer {
    options: NativeOptions,
    configuration: RustTracerConfiguration,
}

impl RustTracer {
    pub fn new(configuration: RustTracerConfiguration) -> RustTracer {
        RustTracer {
            options: NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size(configuration.window_size().get_size()),
                ..Default::default()
            },
            configuration,
        }
    }

    pub fn run(self) -> eframe::Result {
        eframe::run_native(
            "RustTracer",
            self.options,
            Box::new(|cc| {
                Ok(Box::<RustTracerApplication>::new(
                    RustTracerApplication::new(self.configuration, &cc.egui_ctx),
                ))
            }),
        )
    }
}
