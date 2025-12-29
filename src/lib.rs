mod application;
mod core;
mod io;
mod raytracer;
mod raytracing;

pub use application::app::RustTracer;

pub use core::configuration::{
    Height, RenderSize, RustTracerConfiguration, Size, Width, WindowSize,
};
