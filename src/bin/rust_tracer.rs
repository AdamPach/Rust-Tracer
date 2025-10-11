use rust_tracer::{
    Height, RendererConfiguration, RustTracer, RustTracerConfiguration, Size, Width,
};
use std::sync::{Arc, RwLock};

fn main() -> Result<(), eframe::Error> {
    let renderer_config = Arc::new(RwLock::new(RendererConfiguration::new(Size::new(
        Width(720),
        Height(480),
    ))));

    let configuration =
        RustTracerConfiguration::new(Size::new(Width(1200), Height(800)), renderer_config);

    let rust_tracer = RustTracer::new(configuration);

    rust_tracer.run()
}
