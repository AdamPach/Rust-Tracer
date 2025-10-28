mod application;
mod raytracing;
mod renderer;

pub use application::app::RustTracer;
pub use application::configuration::{
    Height, RendererConfiguration, RustTracerConfiguration, Size, Width,
};
