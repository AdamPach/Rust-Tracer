use rust_tracer::{Height, RendererConfiguration, RustTracer, RustTracerConfiguration, Width, WindowSize};

fn main() -> Result<(), eframe::Error> {
    let configuration = RustTracerConfiguration::new(
        WindowSize::new(Width(1200.0), Height(800.0)),
        RendererConfiguration::new(WindowSize::new(Width(720.0), Height(480.0))),
    );
    
    let rust_tracer = RustTracer::new(configuration);

    rust_tracer.run()
}
