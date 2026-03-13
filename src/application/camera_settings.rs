use crate::application::state::CameraState;
use eframe::egui;

pub struct CameraSettingsView<'a> {
    state: &'a mut CameraState,
}

impl <'a>CameraSettingsView<'a> {
    pub fn from_state(camera_state: &'a mut CameraState) -> Self {
        Self {
            state: camera_state,
        }
    }

    pub fn ui<T: FnOnce()>(&mut self, ui: &mut egui::Ui, on_update: T) {
        egui::CollapsingHeader::new("Camera").show(ui, |ui| {
            egui::Grid::new("camera_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    ui.label("View from ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut self.state.position[0]).speed(0.1));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut self.state.position[1]).speed(0.1));
                        ui.label("Z:");
                        ui.add(egui::DragValue::new(&mut self.state.position[2]).speed(0.1));
                    });

                    ui.end_row();
                    ui.label("View at ");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut self.state.view_at[0]).speed(0.1));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut self.state.view_at[1]).speed(0.1));
                        ui.label("Z:");
                        ui.add(egui::DragValue::new(&mut self.state.view_at[2]).speed(0.1));
                    });
                    ui.end_row();

                    ui.label("Field of View ");
                    ui.add(egui::Slider::new(&mut self.state.fov, 10.0..=180.0).suffix("°"));
                    ui.end_row();
                });

            if ui.button("Update Camera").clicked() {
                on_update();
            }
        });
    }
}
