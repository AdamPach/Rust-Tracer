use crate::core::configuration::Size;
use crate::core::geometry::point::Point;

pub struct RaytracerSettings {
    size: Size,
    camera_settings: CameraSettings,
}

impl RaytracerSettings {
    pub fn new(size: Size, camera_settings: CameraSettings) -> Self {
        Self {
            size,
            camera_settings,
        }
    }

    pub fn size(&self) -> Size {
        self.size.clone()
    }

    pub fn camera_settings(&self) -> CameraSettings {
        self.camera_settings.clone()
    }
}

#[derive(Clone)]
pub struct CameraSettings {
    pub position: Point,
}
