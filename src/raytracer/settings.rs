use crate::core::configuration::Size;
use crate::core::geometry::point::Point;

pub struct RaytracerSettings {
    pub size: Size,
    pub camera_view_from: Point,
}

impl RaytracerSettings {
    pub fn new(size: Size, camera_view_from: Point) -> Self {
        Self {
            size,
            camera_view_from,
        }
    }

    pub fn size(&self) -> Size {
        self.size.clone()
    }

    pub fn camera_view_from(&self) -> Point {
        self.camera_view_from.clone()
    }
}
