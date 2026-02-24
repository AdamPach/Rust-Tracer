use crate::application::rendering_thread::RenderingThread;
use crate::application::state::{ApplicationState, SceneState};
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
    file_dialog: FileDialog,
}

impl Application {
    pub fn new(into_state: impl Into<ApplicationState>, ctx: &Context) -> Self {
        let state: ApplicationState = into_state.into();

        let renderer = Raytracer::new(state.clone());

        Self {
            render: ctx.load_texture(
                "Render",
                Render::new(state.render_size().clone()),
                Default::default(),
            ),
            rendering_thread: RenderingThread::new(renderer),
            state,
            file_dialog: FileDialog::new()
                .add_file_filter(
                    "OBJ File",
                    Arc::new(|p| p.extension().unwrap_or_default() == "obj"),
                )
                .default_file_filter("OBJ File"),
        }
    }

    fn try_update_application(&mut self, ctx: &Context) {
        if let Ok(response) = self.rendering_thread.try_recv_render() {
            match response {
                Ok(RaytracerResponse::RenderComplete(render)) => {
                    self.render = ctx.load_texture("Render", render, Default::default());
                }
                Ok(RaytracerResponse::SceneLoaded) => {
                    self.state.scene_state = match &self.state.scene_state {
                        SceneState::Loading(path) => SceneState::Loaded(path.clone()),
                        _ => SceneState::None,
                    }
                }
                _ => {}
            }
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.try_update_application(ctx);

        ctx.request_repaint_after(std::time::Duration::from_millis(16));

        egui::Window::new("Render")
            .default_width(self.state.render_size().get_width().into())
            .default_height(self.state.render_size().get_height().into())
            .show(ctx, |ui| {
                ui.image((self.render.id(), ui.available_size()));
            });

        egui::Window::new("Settings")
            .default_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                self.camera_settings_ui(ui);
                self.scene_settings_ui(ui, ctx);

                egui::CollapsingHeader::new("Rendering")
                    .default_open(false)
                    .show(ui, |ui| if ui.button("Start").clicked() {});
            });
    }
}

impl Application {
    fn camera_settings_ui(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("Camera").show(ui, |ui| {
            egui::Grid::new("camera_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    ui.label("View from ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.position[0])
                                .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.position[1])
                                .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.position[2])
                                .speed(0.1),
                        );
                    });
                    ui.end_row();
                    ui.label("View at ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.view_at[0])
                                .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.view_at[1])
                                .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::new(&mut self.state.camera_state.view_at[2])
                                .speed(0.1),
                        );
                    });
                    ui.end_row();

                    ui.label("Field of View ");
                    ui.add(
                        egui::Slider::new(&mut self.state.camera_state.fov, 10.0..=180.0)
                            .suffix("°"),
                    );
                    ui.end_row();
                });

            if ui.button("Update Camera").clicked() {
                self.rendering_thread
                    .send_command(RaytracerCommand::CameraUpdate(
                        self.state.camera_state.clone().into(),
                    ));
            }
        });
    }

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
                        .send_command(RaytracerCommand::SceneUpdate(
                            SceneLoadingDta::WavefrontObj { path: path.clone() },
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
