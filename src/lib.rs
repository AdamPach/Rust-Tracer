mod application;
mod raytracer;

pub use application::app::RustTracer;
pub use application::configuration::{
    Height, RendererConfiguration, RustTracerConfiguration, Size, Width,
};
