use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::raytracer::RaytracerSettings;
use crate::{RustTracerConfiguration, Size};

#[derive(Clone)]
pub struct ApplicationState {
    size: Size,
    pub camera_setting: CameraSetting,
}

impl ApplicationState {
    pub fn render_size(&self) -> &Size {
        &self.size
    }
}

impl Into<ApplicationState> for RustTracerConfiguration {
    fn into(self) -> ApplicationState {
        ApplicationState {
            size: self.default_render_size().clone(),
            camera_setting: CameraSetting {
                position: [10.0, 0.0, 0.0],
            },
        }
    }
}

impl Into<RaytracerSettings> for ApplicationState {
    fn into(self) -> RaytracerSettings {
        RaytracerSettings::new(
            self.size,
            Point::new(
                X::new(self.camera_setting.position[0]),
                Y::new(self.camera_setting.position[1]),
                Z::new(self.camera_setting.position[2]),
            ),
        )
    }
}

#[derive(Clone)]
pub struct CameraSetting {
    pub position: [f64; 3],
}
