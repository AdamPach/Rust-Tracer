use crate::application::camera_settings::{CameraSettingsView, CameraSettingsViewEvent};
use crate::application::notifications::{Notification, NotificationsView};
use crate::application::rendering_settings::{RenderingSettingsView, RenderingSettingsViewEvent};
use crate::application::rendering_thread::{
    RenderingThread, RenderingThreadCommand, RenderingThreadResponse,
};
use crate::application::scene_settings::{SceneSettingsView, SceneSettingsViewEvent};
use crate::application::state::{ApplicationState, ApplicationStateUpdate, SceneState};
use crate::core::render::Render;
use crate::rendering::{Raytracer, RaytracerCommand, RaytracerResponse, SceneLoadingDta};
use eframe::egui::{Context, TextureHandle};
use eframe::epaint::{ColorImage, ImageData};
use eframe::{Frame, egui};
use egui_file_dialog::FileDialog;
use std::default::Default;
use std::sync::Arc;

pub struct Application {
    render: TextureHandle,
    rendering_thread: RenderingThread,
    state: ApplicationState,
    state_updates: Vec<ApplicationStateUpdate>,
    file_dialog: FileDialog,
}

impl Application {
    pub fn new(into_state: impl Into<ApplicationState>, ctx: &Context) -> Self {
        let state: ApplicationState = into_state.into();

        let render = ctx.load_texture(
            "Render",
            Render::black(state.render_size().clone()),
            Default::default(),
        );

        let file_dialog = FileDialog::new()
            .add_file_filter(
                "OBJ File",
                Arc::new(|p| p.extension().unwrap_or_default() == "obj"),
            )
            .default_file_filter("OBJ File");

        Self {
            render,
            rendering_thread: RenderingThread::new(Raytracer::new(state.clone())),
            state,
            state_updates: vec![],
            file_dialog,
        }
    }

    fn update_application(&mut self, ctx: &Context) {
        while let Some(update) = self.state_updates.pop() {
            match update {
                ApplicationStateUpdate::RemoveNotification(index) => {
                    self.state.remove_notification(index);
                }
                ApplicationStateUpdate::CameraEvent(event) => match event {
                    CameraSettingsViewEvent::ChangePosition(position) => {
                        self.state.change_camera(
                            self.state.camera_state().clone().with_position(position),
                        );
                    }
                    CameraSettingsViewEvent::ChangeViewAt(view_at) => {
                        self.state
                            .change_camera(self.state.camera_state().clone().with_view_at(view_at));
                    }
                    CameraSettingsViewEvent::ChangeFov(fov) => {
                        self.state
                            .change_camera(self.state.camera_state().clone().with_fov(fov));
                    }
                    CameraSettingsViewEvent::UpdateCamera => {
                        self.rendering_thread
                            .send_command(RenderingThreadCommand::SendCommand(
                                RaytracerCommand::CameraUpdate(
                                    self.state.camera_state().clone().into(),
                                ),
                            ));
                    }
                },
                ApplicationStateUpdate::SceneEvent(event) => match event {
                    SceneSettingsViewEvent::LoadScene => self.file_dialog.pick_file(),
                },
                ApplicationStateUpdate::RendererEvent(event) => match event {
                    RenderingSettingsViewEvent::StartRendering => {
                        self.rendering_thread
                            .send_command(RenderingThreadCommand::StartRendering);
                    }
                    RenderingSettingsViewEvent::StopRendering => {
                        self.rendering_thread
                            .send_command(RenderingThreadCommand::StopRendering);
                    }
                    RenderingSettingsViewEvent::ResetRendering => {
                        self.rendering_thread
                            .send_command(RenderingThreadCommand::SendCommand(
                                RaytracerCommand::ClearAccumulator,
                            ));
                    }
                },
            }
        }

        while let Ok(response) = self.rendering_thread.try_read_responses() {
            match response {
                Ok(RenderingThreadResponse::CommandResponse(RaytracerResponse::SceneLoaded)) => {
                    self.state
                        .change_scene_state(match self.state.scene_state() {
                            SceneState::Loading(path) => SceneState::Loaded(path.clone()),
                            _ => SceneState::None,
                        });
                }
                Ok(RenderingThreadResponse::CommandResponse(RaytracerResponse::CameraUpdated)) => {
                    self.state
                        .add_notification(Notification::ok("Camera updated".to_string()));
                }
                Ok(RenderingThreadResponse::CommandResponse(
                    RaytracerResponse::AccumulatorCleared,
                )) => {
                    self.render = ctx.load_texture(
                        "Render",
                        Render::black(self.state.render_size().clone()),
                        Default::default(),
                    );
                }
                Ok(RenderingThreadResponse::RenderingStarted) => {
                    self.state.change_rendering(true);
                }
                Ok(RenderingThreadResponse::RenderingStopped) => {
                    self.state.change_rendering(false);
                }
                Err(_) => {}
            }
        }

        if let Some(render) = self.rendering_thread.get_last_render() {
            self.render = ctx.load_texture("Render", render, Default::default());
        }

        self.state.retain_notifications();
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_application(ctx);

        ctx.request_repaint_after(std::time::Duration::from_millis(16));

        egui::Window::new("Render")
            .default_width(self.state.render_size().get_width().into())
            .default_height(self.state.render_size().get_height().into())
            .show(ctx, |ui| {
                ui.image((self.render.id(), ui.available_size()));
            });

        egui::Window::new("Settings")
            .default_width(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                NotificationsView::new(&self.state.notifications()).ui(ui, |index| {
                    self.state_updates
                        .push(ApplicationStateUpdate::RemoveNotification(index));
                });

                CameraSettingsView::new(&self.state.camera_state()).ui(ui, |event| {
                    self.state_updates
                        .push(ApplicationStateUpdate::CameraEvent(event));
                });

                SceneSettingsView::new(&self.state.scene_state()).ui(ui, |event| match event {
                    SceneSettingsViewEvent::LoadScene => {
                        self.state_updates
                            .push(ApplicationStateUpdate::SceneEvent(event));
                    }
                });

                self.file_dialog.update(ctx);

                if let Some(path) = self.file_dialog.take_picked() {
                    self.state.change_scene_state(SceneState::Loading(
                        path.file_name().unwrap().to_str().unwrap().to_string(),
                    ));

                    self.rendering_thread
                        .send_command(RenderingThreadCommand::SendCommand(
                            RaytracerCommand::SceneUpdate(SceneLoadingDta::WavefrontObj {
                                path: path.clone(),
                            }),
                        ));
                }

                RenderingSettingsView::new(&self.state.rendering()).ui(ui, |event| {
                    self.state_updates
                        .push(ApplicationStateUpdate::RendererEvent(event));
                })
            });
    }
}

impl From<Render> for ImageData {
    fn from(render: Render) -> Self {
        let render_data = render.get_render_data();

        ColorImage::from_rgba_unmultiplied(
            [
                render_data.0.get_width().get(),
                render_data.0.get_height().get(),
            ],
            &render_data.1,
        )
        .into()
    }
}
