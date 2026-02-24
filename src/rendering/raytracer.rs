use crate::core::render::{Render, RenderAccumulator};
use crate::rendering::raytracer_renderer::RaytracerRenderer;
use crate::rendering::settings::RaytracerSettings;
use crate::rendering::threadpool::ThreadPool;
use crate::rendering::{CameraSettings, RaytracerCommand, RaytracerResponse};
use crate::raytracing::{Camera, SceneBuilder, SceneDescriptor};

pub struct Raytracer {
    settings: RaytracerSettings,
    rendering_thread_pool: ThreadPool<RaytracerRenderer>,
    accumulator: RenderAccumulator,
}

impl Raytracer {
    pub fn new(state: impl Into<RaytracerSettings>) -> Self {
        let settings: RaytracerSettings = state.into();

        let camera_settings = settings.camera_settings();

        let camera = Camera::new(settings.size(), camera_settings);

        let scene = SceneDescriptor::default().scene();

        let renderer = RaytracerRenderer::new(scene, camera);

        let rendering_thread_pool = ThreadPool::new(32, renderer);

        let accumulator = RenderAccumulator::new(settings.size().clone());

        Self {
            settings,
            rendering_thread_pool,
            accumulator,
        }
    }

    pub fn send_command(&mut self, command: RaytracerCommand) -> anyhow::Result<RaytracerResponse> {
        match command {
            RaytracerCommand::RenderFrame => {
                Ok(RaytracerResponse::RenderComplete(self.render_image()))
            }
            RaytracerCommand::CameraUpdate(settings) => {
                self.set_camera(settings)?;
                Ok(RaytracerResponse::RendererUpdated)
            }
            RaytracerCommand::SceneUpdate(builder) => {
                self.set_scene(builder.loader())?;
                Ok(RaytracerResponse::SceneLoaded)
            }
        }
    }

    fn set_camera(&mut self, camera_settings: CameraSettings) -> anyhow::Result<()> {
        let camera = Camera::new(self.settings.size(), camera_settings);

        self.rendering_thread_pool.set_camera(camera);
        self.accumulator.clear();

        Ok(())
    }

    fn set_scene<T: SceneBuilder>(&mut self, scene_builder: T) -> anyhow::Result<()> {
        let scene = scene_builder.build_scene()?;

        self.rendering_thread_pool.set_scene(scene);
        self.accumulator.clear();

        Ok(())
    }

    fn render_image(&mut self) -> Render {
        let mut test = self.accumulator.accumulated_render();

        test = self.rendering_thread_pool.render(test);

        test.get_render()
    }
}
