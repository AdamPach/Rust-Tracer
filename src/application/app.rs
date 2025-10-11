use crate::application::configuration::RustTracerConfiguration;
use crate::application::gui::RustTracerApplication;
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
                    .with_inner_size(configuration.size().get_size()),
                ..Default::default()
            },
            configuration,
        }
    }

    pub fn run(self) -> eframe::Result {
        eframe::run_native(
            "RustTracer",
            self.options,
            Box::new(|_cc| {
                Ok(Box::<RustTracerApplication>::new(
                    RustTracerApplication::new(self.configuration.into()),
                ))
            }),
        )
    }
}
