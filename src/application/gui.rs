use crate::application::rendering_thread::RenderingThread;
use crate::application::state::ApplicationState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::Render;
use crate::io::wavefront::WavefrontLoader;
use crate::raytracer::{Raytracer, RaytracerCommand};
use eframe::egui::{Context, TextureHandle};
use eframe::epaint::{ColorImage, ImageData};
use eframe::{Frame, egui};
use std::default::Default;
use std::env;

pub struct Application {
    render: TextureHandle,
    rendering_thread: RenderingThread,
    state: ApplicationState,
}

impl Application {
    pub fn new(into_state: impl Into<ApplicationState>, ctx: &Context) -> Self {
        let state: ApplicationState = into_state.into();

        let mut renderer = Raytracer::new(state.clone());

        let result = renderer.set_scene(WavefrontLoader::new(
            env::current_dir().unwrap().join("assets/cubes.obj"),
        ));

        if let Err(e) = result {
            println!("[ERROR]: Loading a scene failed with errors!");
            println!("{:#}", e);
        }

        Self {
            render: ctx.load_texture(
                "Render",
                Render::new(state.render_size().clone()),
                Default::default(),
            ),
            rendering_thread: RenderingThread::new(renderer),
            state,
        }
    }

    fn try_update_render(&mut self, ctx: &Context) {
        if let Ok(render) = self.rendering_thread.try_recv_render() {
            self.render = ctx.load_texture("Render", render, Default::default());
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.try_update_render(ctx);

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
                ui.label("Camera");

                egui::Grid::new("camera")
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
                    });

                if ui.button("Update Camera").clicked() {
                    self.rendering_thread
                        .send_command(RaytracerCommand::CameraUpdate {
                            position: Point::new(
                                X::new(self.state.camera_state.position[0]),
                                Y::new(self.state.camera_state.position[1]),
                                Z::new(self.state.camera_state.position[2]),
                            ),
                        });
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
