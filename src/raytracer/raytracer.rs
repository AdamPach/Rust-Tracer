use crate::core::render::Render;
use crate::raytracer::raytracer_renderer::RaytracerRenderer;
use crate::raytracer::settings::RaytracerSettings;
use crate::raytracer::threadpool::ThreadPool;
use crate::raytracer::{CameraSettings, RaytracerCommand, RaytracerResponse};
use crate::raytracing::{Camera, SceneBuilder, SceneDescriptor};

pub struct Raytracer {
    state: RaytracerSettings,
    rendering_thread_pool: ThreadPool<RaytracerRenderer>,
}

impl Raytracer {
    pub fn new(state: impl Into<RaytracerSettings>) -> Self {
        let state: RaytracerSettings = state.into();

        let camera_settings = state.camera_settings();

        let camera = Camera::new(
            state.size(),
            camera_settings.position,
            std::f64::consts::FRAC_PI_4,
        );

        let scene = SceneDescriptor::default().scene();

        let renderer = RaytracerRenderer::new(scene, camera);

        let rendering_thread_pool = ThreadPool::new(32, renderer);

        Self {
            state,
            rendering_thread_pool,
        }
    }

    pub fn set_scene<T: SceneBuilder>(&self, scene_builder: T) -> anyhow::Result<()> {
        let scene = scene_builder.build_scene()?;

        self.rendering_thread_pool.set_scene(scene);

        Ok(())
    }

    pub fn send_command(&mut self, command: RaytracerCommand) -> RaytracerResponse {
        match command {
            RaytracerCommand::RenderFrame => RaytracerResponse::RenderComplete(self.render_image()),
            RaytracerCommand::CameraUpdate(settings) => {
                self.set_camera(settings).unwrap();
                RaytracerResponse::SettingsUpdated
            }
        }
    }

    fn set_camera(&mut self, camera_settings: CameraSettings) -> anyhow::Result<()> {
        let camera = Camera::new(
            self.state.size(),
            camera_settings.position,
            std::f64::consts::FRAC_PI_4,
        );

        self.rendering_thread_pool.set_camera(camera);

        Ok(())
    }

    fn render_image(&self) -> Render {
        let render = Render::new(self.state.size().clone());

        self.rendering_thread_pool.render(render)
    }
}
