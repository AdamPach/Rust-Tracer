use crate::RendererState;
use crate::core::render::{PixelPosition, Render, RenderPixel};
use crate::raytracer::raytracer_configuration::RayTracerConfiguration;
use crate::raytracer::threadpool::{Renderer, ThreadPool};
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene, SceneBuilder, SceneDescriptor};
use std::sync::Arc;

pub struct Raytracer {
    configuration: RayTracerConfiguration,
    rendering_thread_pool: ThreadPool<RaytracerRenderer>,
    renderer: Arc<RaytracerRenderer>,
}

impl Raytracer {
    pub fn new(state: RendererState) -> Self {
        let configuration: RayTracerConfiguration = state.into();

        let camera = Camera::new(
            configuration.size().get_width(),
            configuration.size().get_height(),
            std::f64::consts::FRAC_PI_4,
        );

        let scene = SceneDescriptor::default().scene();

        let renderer = Arc::new(RaytracerRenderer { scene, camera });

        let rendering_thread_pool = ThreadPool::new(32, renderer.clone());

        Self {
            configuration,
            rendering_thread_pool,
            renderer,
        }
    }

    pub fn set_scene<T: SceneBuilder>(&mut self, scene_builder: T) -> anyhow::Result<()> {
        let scene = scene_builder.build_scene()?;
        let camera = self.renderer.camera.clone();

        self.renderer = Arc::new(RaytracerRenderer { scene, camera });

        self.rendering_thread_pool
            .set_new_renderer(self.renderer.clone());

        Ok(())
    }

    pub fn render_image(&self) -> Render {
        let render = Render::new(self.configuration.size().clone());

        self.rendering_thread_pool.render(render)
    }
}

struct RaytracerRenderer {
    scene: Scene,
    camera: Camera,
}

impl Renderer for RaytracerRenderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel {
        let (x, y) = position.get_pixel_coordinates();

        let ray = self.camera.generate_ray(x, y);

        let mut output_color =
            MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));

        if let Some(ray_hit) = self.scene.find_intersection(ray) {
            if let Some(material) = self.scene.get_material(ray_hit.material_id()) {
                output_color = crate::raytracer::shading::shade_hit_with_material(material);
            }
        }

        position.create_render_pixel(output_color)
    }
}
