pub mod color;
mod pixels;
mod render;

pub use pixels::{PixelX, PixelY};
pub use render::{
    AccumulatedRender, PixelPosition, Render, RenderAccumulator, RenderIterator, RenderPixel,
    RenderState,
};
