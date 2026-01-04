use crate::core::render::Render;
use crate::raytracer::CameraSettings;

pub enum RaytracerCommand {
    RenderFrame,
    CameraUpdate(CameraSettings),
}

pub enum RaytracerResponse {
    RenderComplete(Render),
    RendererUpdated,
}
