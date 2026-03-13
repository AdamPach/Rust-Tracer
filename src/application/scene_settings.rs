use crate::application::state::SceneState;
use eframe::egui;

pub enum SceneSettingsViewEvent {
    LoadScene,
}

pub struct SceneSettingsView<'a> {
    state: &'a SceneState,
}

impl<'a> SceneSettingsView<'a> {
    pub fn new(state: &'a SceneState) -> Self {
        Self { state }
    }

    pub fn ui<T: FnOnce(SceneSettingsViewEvent)>(&self, ui: &mut egui::Ui, on_event: T) {
        egui::CollapsingHeader::new("Scene")
            .default_open(false)
            .show(ui, |ui| {
                egui::Grid::new("scene_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(self.state.string_status());

                        ui.end_row();

                        if ui.button("Load OBJ Scene").clicked() {
                            on_event(SceneSettingsViewEvent::LoadScene);
                        }

                        if ui.button("Remove Scene").clicked() {}
                    });
            });
    }
}
