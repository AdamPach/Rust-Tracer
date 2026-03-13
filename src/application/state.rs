use crate::application::camera_settings::CameraSettingsViewEvent;
use crate::application::notifications::Notification;
use crate::application::rendering_settings::RenderingSettingsViewEvent;
use crate::application::scene_settings::SceneSettingsViewEvent;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::rendering::CameraSettings;
use crate::rendering::RaytracerSettings;
use crate::{RustTracerConfiguration, Size};

pub enum ApplicationStateUpdate {
    RemoveNotification(usize),
    CameraEvent(CameraSettingsViewEvent),
    SceneEvent(SceneSettingsViewEvent),
    RendererEvent(RenderingSettingsViewEvent),
}

#[derive(Clone)]
pub struct ApplicationState {
    size: Size,
    camera_state: CameraState,
    scene_state: SceneState,
    rendering: bool,
    notifications: Vec<Notification>,
}

impl ApplicationState {
    pub fn render_size(&self) -> &Size {
        &self.size
    }

    pub fn camera_state(&self) -> &CameraState {
        &self.camera_state
    }

    pub fn change_camera(&mut self, camera_state: CameraState) {
        self.camera_state = camera_state;
    }

    pub fn rendering(&self) -> &bool {
        &self.rendering
    }

    pub fn change_rendering(&mut self, rendering: bool) {
        self.rendering = rendering;
    }

    pub fn scene_state(&self) -> &SceneState {
        &self.scene_state
    }

    pub fn change_scene_state(&mut self, scene_state: SceneState) {
        self.scene_state = scene_state;
    }

    pub fn notifications(&self) -> &Vec<Notification> {
        &self.notifications
    }

    pub fn remove_notification(&mut self, index: usize) {
        if index < self.notifications.len() {
            self.notifications.remove(index);
        }
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn retain_notifications(&mut self) {
        self.notifications.retain(|n| !n.is_expired());
    }
}

impl Into<ApplicationState> for RustTracerConfiguration {
    fn into(self) -> ApplicationState {
        let notifications = vec![Notification::info(
            "Welcome to the Raytracer! This is test message to see how huge text is displayed!"
                .to_string(),
        )];

        ApplicationState {
            size: self.default_render_size().clone(),
            scene_state: SceneState::None,
            camera_state: CameraState {
                position: [10.0, 0.0, 0.0],
                view_at: [0.0, 0.0, 0.0],
                fov: 60.0,
            },
            rendering: false,
            notifications,
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
    position: [f64; 3],
    view_at: [f64; 3],
    fov: f64,
}

impl CameraState {
    pub fn new(position: [f64; 3], view_at: [f64; 3], fov: f64) -> Self {
        Self {
            position,
            view_at,
            fov,
        }
    }

    pub fn position(&self) -> [f64; 3] {
        self.position
    }

    pub fn view_at(&self) -> [f64; 3] {
        self.view_at
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn with_position(mut self, position: [f64; 3]) -> Self {
        self.position = position;
        self
    }

    pub fn with_view_at(mut self, view_at: [f64; 3]) -> Self {
        self.view_at = view_at;
        self
    }

    pub fn with_fov(mut self, fov: f64) -> Self {
        self.fov = fov;
        self
    }
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
    Loading(String),
    Loaded(String),
}

impl SceneState {
    pub fn string_status(&self) -> String {
        match self {
            SceneState::None => "No scene loaded".to_string(),
            SceneState::Loading(_) => "Loading Scene...".to_string(),
            SceneState::Loaded(path) => format!("Loaded {}", path),
        }
    }
}
