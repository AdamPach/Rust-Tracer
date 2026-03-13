use crate::application::state::CameraState;
use eframe::egui;

pub enum CameraSettingsViewEvent {
    ChangePosition([f64; 3]),
    ChangeViewAt([f64; 3]),
    ChangeFov(f64),
    UpdateCamera,
}

pub struct CameraSettingsView<'a> {
    state: &'a CameraState,
}

impl<'a> CameraSettingsView<'a> {
    pub fn new(camera_state: &'a CameraState) -> Self {
        Self {
            state: camera_state,
        }
    }

    pub fn ui<T: FnOnce(CameraSettingsViewEvent)>(&mut self, ui: &mut egui::Ui, on_event: T) {
        egui::CollapsingHeader::new("Camera").show(ui, |ui| {
            let mut event = None;

            egui::Grid::new("camera_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    let mut position = self.state.position();

                    ui.label("View from ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    position[0] = value;
                                    event = Some(CameraSettingsViewEvent::ChangePosition(position));
                                }
                                position[0]
                            })
                            .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    position[1] = value;
                                    event = Some(CameraSettingsViewEvent::ChangePosition(position));
                                }
                                position[1]
                            })
                            .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    position[2] = value;
                                    event = Some(CameraSettingsViewEvent::ChangePosition(position));
                                }
                                position[2]
                            })
                            .speed(0.1),
                        );
                    });

                    let mut view_at = self.state.view_at();

                    ui.end_row();
                    ui.label("View at ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    view_at[0] = value;
                                    event = Some(CameraSettingsViewEvent::ChangeViewAt(view_at));
                                }
                                view_at[0]
                            })
                            .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    view_at[1] = value;
                                    event = Some(CameraSettingsViewEvent::ChangeViewAt(view_at));
                                }
                                view_at[1]
                            })
                            .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::from_get_set(|setter| {
                                if let Some(value) = setter {
                                    view_at[2] = value;
                                    event = Some(CameraSettingsViewEvent::ChangeViewAt(view_at));
                                }
                                view_at[2]
                            })
                            .speed(0.1),
                        );
                    });
                    ui.end_row();

                    let mut fov = self.state.fov();

                    ui.label("Field of View ");
                    ui.add(
                        egui::Slider::from_get_set(10.0..=180.0, |setter| {
                            if let Some(value) = setter {
                                if value != fov {
                                    fov = value;
                                    event = Some(CameraSettingsViewEvent::ChangeFov(fov));
                                }
                            }
                            fov
                        })
                        .suffix("°"),
                    );
                    ui.end_row();
                });

            if ui.button("Update Camera").clicked() {
                event = Some(CameraSettingsViewEvent::UpdateCamera);
            }

            if let Some(event) = event {
                on_event(event);
            }
        });
    }
}
