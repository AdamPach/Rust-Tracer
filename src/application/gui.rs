use crate::application::camera_settings::CameraSettingsView;
use crate::application::notifications::{Notification, NotificationsView};
use crate::application::rendering_thread::{
    RenderingThread, RenderingThreadCommand, RenderingThreadResponse,
};
use crate::application::state::{ApplicationState, ApplicationStateUpdate, SceneState};
use crate::core::render::Render;
use crate::rendering::{Raytracer, RaytracerCommand, RaytracerResponse, SceneLoadingDta};
use eframe::egui::{Context, TextureHandle, Ui};
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

        Self {
            render: ctx.load_texture(
                "Render",
                Render::black(state.render_size().clone()),
                Default::default(),
            ),
            rendering_thread: RenderingThread::new(Raytracer::new(state.clone())),
            state,
            state_updates: vec![],
            file_dialog: FileDialog::new()
                .add_file_filter(
                    "OBJ File",
                    Arc::new(|p| p.extension().unwrap_or_default() == "obj"),
                )
                .default_file_filter("OBJ File"),
        }
    }

    fn update_application(&mut self, ctx: &Context) {
        while let Some(update) = self.state_updates.pop() {
            match update {
                ApplicationStateUpdate::RemoveNotification(index) => {
                    if index < self.state.notifications.len() {
                        self.state.notifications.remove(index);
                    }
                },
                ApplicationStateUpdate::CameraUpdate => {
                    self.rendering_thread
                        .send_command(RenderingThreadCommand::SendCommand(
                            RaytracerCommand::CameraUpdate(self.state.camera_state.clone().into()),
                        ));
                }
            }
        }

        while let Ok(response) = self.rendering_thread.try_read_responses() {
            match response {
                Ok(RenderingThreadResponse::CommandResponse(RaytracerResponse::SceneLoaded)) => {
                    self.state.scene_state = match &self.state.scene_state {
                        SceneState::Loading(path) => SceneState::Loaded(path.clone()),
                        _ => SceneState::None,
                    }
                }
                Ok(RenderingThreadResponse::CommandResponse(RaytracerResponse::CameraUpdated)) => {
                    self.state.notifications.push(Notification::ok("Camera updated".to_string()));
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
                    self.state.rendering = true;
                }
                Ok(RenderingThreadResponse::RenderingStopped) => {
                    self.state.rendering = false;
                }
                Err(_) => {}
            }
        }

        if let Some(render) = self.rendering_thread.get_last_render() {
            self.render = ctx.load_texture("Render", render, Default::default());
        }

        self.state.notifications.retain(|n| !n.is_expired())
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
                NotificationsView::new(&self.state.notifications).ui(ui, |index| {
                    self.state_updates
                        .push(ApplicationStateUpdate::RemoveNotification(index));
                });

                CameraSettingsView::from_state(&mut self.state.camera_state).ui(ui, ||{
                        self.state_updates.push(ApplicationStateUpdate::CameraUpdate);
                });

                self.scene_settings_ui(ui, ctx);

                egui::CollapsingHeader::new("Rendering")
                    .default_open(false)
                    .show(ui, |ui| {
                        let run_button_text = if self.state.rendering {
                            "Stop"
                        } else {
                            "Start"
                        };

                        if ui.button(run_button_text).clicked() {
                            if self.state.rendering {
                                self.rendering_thread
                                    .send_command(RenderingThreadCommand::StopRendering);
                            } else {
                                self.rendering_thread
                                    .send_command(RenderingThreadCommand::StartRendering);
                            }
                        }

                        if ui.button("Reset Render").clicked() {
                            self.rendering_thread.send_command(
                                RenderingThreadCommand::SendCommand(
                                    RaytracerCommand::ClearAccumulator,
                                ),
                            );
                        }
                    });
            });
    }
}

impl Application {
    fn scene_settings_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::CollapsingHeader::new("Scene")
            .default_open(false)
            .show(ui, |ui| {
                egui::Grid::new("scene_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(self.state.scene_state.string_status());

                        ui.end_row();

                        if ui.button("Load OBJ Scene").clicked() {
                            self.file_dialog.pick_file()
                        }

                        if ui.button("Remove Scene").clicked() {}
                    });

                self.file_dialog.update(ctx);

                if let Some(path) = self.file_dialog.take_picked() {
                    self.state.scene_state = SceneState::Loading(
                        path.file_name().unwrap().to_str().unwrap().to_string(),
                    );

                    self.rendering_thread
                        .send_command(RenderingThreadCommand::SendCommand(
                            RaytracerCommand::SceneUpdate(SceneLoadingDta::WavefrontObj {
                                path: path.clone(),
                            }),
                        ));
                }
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
