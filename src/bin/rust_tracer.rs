use rust_tracer::{
    Height, RendererConfiguration, RustTracer, RustTracerConfiguration, Size, Width,
};

fn main() -> Result<(), eframe::Error> {
    let renderer_config = RendererConfiguration::new(Size::new(Width(720), Height(480)));

    let configuration =
        RustTracerConfiguration::new(Size::new(Width(1200), Height(600)), renderer_config);

    let rust_tracer = RustTracer::new(configuration);

    rust_tracer.run()
}
