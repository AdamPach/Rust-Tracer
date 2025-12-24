use crate::core::configuration::RendererState;
use crate::core::render::{PixelPosition, Render, RenderPixel};
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene, SceneBuilder};
use crate::renderer::raytracer::raytracer_configuration::RayTracerConfiguration;
use crate::renderer::raytracer::rendering_threadpool::{RenderingThreadPool, ThreadPoolRenderer};
use crate::renderer::raytracer::shading::shade_hit_with_material;
use std::sync::Arc;

pub struct RayTracer {
    configuration: RayTracerConfiguration,
    rendering_thread_pool: RenderingThreadPool,
}

impl RayTracer {
    pub fn new<T: SceneBuilder>(state: RendererState, scene_builder: T) -> Self {
        let configuration: RayTracerConfiguration = state.into();

        let camera = Camera::new(
            configuration.size().get_width(),
            configuration.size().get_height(),
            std::f64::consts::FRAC_PI_4,
        );

        let scene = scene_builder.build_scene();

        let renderer = Arc::new(RaytracerRenderer { scene, camera });

        let rendering_thread_pool = RenderingThreadPool::new(32, renderer);

        Self {
            configuration,
            rendering_thread_pool,
        }
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

impl ThreadPoolRenderer for RaytracerRenderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel {
        let (x, y) = position.get_pixel_coordinates();

        let ray = self.camera.generate_ray(x, y);

        let mut output_color =
            MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));

        if let Some(ray_hit) = self.scene.find_intersection(ray) {
            if let Some(material) = self.scene.get_material(ray_hit.material_id()) {
                output_color = shade_hit_with_material(material);
            }
        }

        position.create_render_pixel(output_color)
    }
}
