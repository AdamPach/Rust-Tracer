mod application;
mod core;
mod raytracing;
mod renderer;

pub use application::app::RustTracer;
pub use core::configuration::{
    Height, RenderSize, RendererState, RustTracerConfiguration, Size, Width, WindowSize,
};
