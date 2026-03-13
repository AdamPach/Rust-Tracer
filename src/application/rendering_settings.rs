use eframe::egui;

pub enum RenderingSettingsViewEvent {
    StartRendering,
    StopRendering,
    ResetRendering,
}
pub struct RenderingSettingsView<'a> {
    rendering: &'a bool,
}

impl<'a> RenderingSettingsView<'a> {
    pub fn new(rendering: &'a bool) -> Self {
        RenderingSettingsView { rendering }
    }

    pub fn ui<T: FnOnce(RenderingSettingsViewEvent)>(&self, ui: &mut egui::Ui, on_event: T) {
        egui::CollapsingHeader::new("Rendering")
            .default_open(false)
            .show(ui, |ui| {
                let run_button_text = if *self.rendering { "Stop" } else { "Start" };

                let mut event = None;

                if ui.button(run_button_text).clicked() {
                    if *self.rendering {
                        event = Some(RenderingSettingsViewEvent::StopRendering);
                    } else {
                        event = Some(RenderingSettingsViewEvent::StartRendering);
                    }
                }

                if ui.button("Reset Render").clicked() {
                    event = Some(RenderingSettingsViewEvent::ResetRendering);
                }

                if let Some(event) = event {
                    on_event(event);
                }
            });
    }
}
