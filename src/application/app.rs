use crate::application::gui::{RustTracerApplication};
use eframe::{NativeOptions, egui};
use crate::application::configuration::{RustTracerConfiguration};

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
            Box::new(|cc| {
                Ok(Box::<RustTracerApplication>::new(
                    RustTracerApplication::new(cc, self.configuration.into()),
                ))
            }),
        )
    }
}
