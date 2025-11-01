use rust_tracer::{
    Height, RenderSize, RustTracer, RustTracerConfiguration, Size, Width, WindowSize,
};

fn main() -> Result<(), eframe::Error> {
    let configuration = RustTracerConfiguration::new(
        WindowSize::new(Size::new(Width::new(1200), Height::new(600))),
        RenderSize::new(Size::new(Width::new(720), Height::new(480))),
    );

    let rust_tracer = RustTracer::new(configuration);

    rust_tracer.run()
}
