use crate::core::render::{Render, RenderAccumulator};
use crate::raytracing::{Camera, SceneBuilder, SceneDescriptor};
use crate::rendering::raytracer_renderer::RaytracerRenderer;
use crate::rendering::settings::RaytracerSettings;
use crate::rendering::threadpool::ThreadPool;
use crate::rendering::{CameraSettings, RaytracerCommand, RaytracerResponse};

pub enum Raytracer {
    Running(RunningRaytracer),
    NotRunning(NonRunningRaytracer),
}

impl Raytracer {
    pub fn new(settings: impl Into<RaytracerSettings>) -> Self {
        let settings: RaytracerSettings = settings.into();

        let camera_settings = settings.camera_settings();

        let camera = Camera::new(settings.size(), camera_settings);

        let scene = SceneDescriptor::default().scene();

        let renderer = RaytracerRenderer::new(scene, camera);

        let accumulator = RenderAccumulator::new(settings.size().clone());

        Raytracer::NotRunning(NonRunningRaytracer {
            settings,
            renderer,
            accumulator,
        })
    }

    pub fn run(self) -> Self {
        match self {
            Raytracer::Running(raytracer) => Raytracer::Running(raytracer),
            Raytracer::NotRunning(raytracer) => {
                let rendering_thread_pool = ThreadPool::new(32, raytracer.renderer);

                Raytracer::Running(RunningRaytracer {
                    settings: raytracer.settings,
                    rendering_thread_pool,
                    accumulator: raytracer.accumulator,
                })
            }
        }
    }

    pub fn stop(self) -> Self {
        match self {
            Raytracer::Running(raytracer) => Raytracer::NotRunning(NonRunningRaytracer {
                settings: raytracer.settings,
                renderer: raytracer.rendering_thread_pool.stop(),
                accumulator: raytracer.accumulator,
            }),
            Raytracer::NotRunning(raytracer) => Raytracer::NotRunning(raytracer),
        }
    }

    pub fn send_command(&mut self, command: RaytracerCommand) -> anyhow::Result<RaytracerResponse> {
        match (self, command) {
            (Raytracer::Running(raytracer), RaytracerCommand::SceneUpdate(scene)) => {
                raytracer.set_scene(scene.loader())?;
                Ok(RaytracerResponse::SceneLoaded)
            }
            (Raytracer::Running(raytracer), RaytracerCommand::CameraUpdate(camera)) => {
                raytracer.set_camera(camera)?;
                Ok(RaytracerResponse::RendererUpdated)
            }
            (Raytracer::NotRunning(raytracer), RaytracerCommand::SceneUpdate(scene)) => {
                raytracer.set_scene(scene.loader())?;
                Ok(RaytracerResponse::SceneLoaded)
            }
            (Raytracer::NotRunning(raytracer), RaytracerCommand::CameraUpdate(camera)) => {
                raytracer.set_camera(camera)?;
                Ok(RaytracerResponse::RendererUpdated)
            }
        }
    }
}

pub struct RunningRaytracer {
    settings: RaytracerSettings,
    rendering_thread_pool: ThreadPool<RaytracerRenderer>,
    accumulator: RenderAccumulator,
}

pub struct NonRunningRaytracer {
    settings: RaytracerSettings,
    renderer: RaytracerRenderer,
    accumulator: RenderAccumulator,
}

impl NonRunningRaytracer {
    fn set_camera(&mut self, camera_settings: CameraSettings) -> anyhow::Result<()> {
        let camera = Camera::new(self.settings.size(), camera_settings);

        self.renderer.set_camera(camera);
        self.accumulator.clear();

        Ok(())
    }

    fn set_scene<T: SceneBuilder>(&mut self, scene_builder: T) -> anyhow::Result<()> {
        let scene = scene_builder.build_scene()?;

        self.renderer.set_scene(scene);
        self.accumulator.clear();

        Ok(())
    }
}

impl RunningRaytracer {
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

    pub fn render_image(&mut self) -> Render {
        let mut render = self.accumulator.accumulated_render();

        render = self.rendering_thread_pool.render(render);

        render.get_render()
    }
}
