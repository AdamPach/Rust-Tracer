use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::rendering::CameraSettings;
use crate::rendering::RaytracerSettings;
use crate::{RustTracerConfiguration, Size};
use std::path::PathBuf;

#[derive(Clone)]
pub struct ApplicationState {
    size: Size,
    pub camera_state: CameraState,
    pub scene_state: SceneState,
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
            scene_state: SceneState::None,
            camera_state: CameraState {
                position: [10.0, 0.0, 0.0],
                view_at: [0.0, 0.0, 0.0],
                fov: 60.0,
            },
        }
    }
}

impl Into<RaytracerSettings> for ApplicationState {
    fn into(self) -> RaytracerSettings {
        RaytracerSettings::new(self.size, self.camera_state.into())
    }
}

#[derive(Clone)]
pub struct CameraState {
    pub position: [f64; 3],
    pub view_at: [f64; 3],
    pub fov: f64,
}

impl Into<CameraSettings> for CameraState {
    fn into(self) -> CameraSettings {
        let position = Point::new(
            X::new(self.position[0]),
            Y::new(self.position[1]),
            Z::new(self.position[2]),
        );

        let view_at = Point::new(
            X::new(self.view_at[0]),
            Y::new(self.view_at[1]),
            Z::new(self.view_at[2]),
        );

        CameraSettings {
            position,
            view_at,
            fov: self.fov * std::f64::consts::PI / 180.0,
        }
    }
}

#[derive(Clone)]
pub enum SceneState {
    None,
    Loading(PathBuf),
    Loaded(PathBuf),
}

impl SceneState {
    pub fn string_status(&self) -> String {
        match self {
            SceneState::None => "No scene loaded".to_string(),
            SceneState::Loading(_) => "Loading Scene...".to_string(),
            SceneState::Loaded(path) => format!("Loaded {}", path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown file")),
        }
    }
}