use crate::core::geometry::point::Point;
use crate::core::render::Render;

pub enum RaytracerCommand {
    RenderFrame,
    CameraUpdate { position: Point },
}

pub enum RaytracerResponse {
    RenderComplete(Render),
    SettingsUpdated,
}
