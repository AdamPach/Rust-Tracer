use rust_tracer::{RustTracer, RustTracerConfiguration};

fn main() -> Result<(), eframe::Error> {
    let configuration = RustTracerConfiguration {
        render_width: 720.0,
        settings_width: 250.0,
        height: 500.0,
    };

    let rust_tracer = RustTracer::new(configuration);

    rust_tracer.run()
}
