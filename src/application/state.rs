use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::raytracer::CameraSettings;
use crate::raytracer::RaytracerSettings;
use crate::{RustTracerConfiguration, Size};

#[derive(Clone)]
pub struct ApplicationState {
    size: Size,
    pub camera_state: CameraState,
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
            camera_state: CameraState {
                position: [10.0, 0.0, 0.0],
            },
        }
    }
}

impl Into<RaytracerSettings> for ApplicationState {
    fn into(self) -> RaytracerSettings {
        RaytracerSettings::new(
            self.size,
            CameraSettings {
                position: Point::new(
                    X::new(self.camera_state.position[0]),
                    Y::new(self.camera_state.position[1]),
                    Z::new(self.camera_state.position[2]),
                ),
            },
        )
    }
}

#[derive(Clone)]
pub struct CameraState {
    pub position: [f64; 3],
}
