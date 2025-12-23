use crate::core::configuration::RendererState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::{PixelPosition, Render, RenderPixel, RenderState};
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene, Triangle, TriangulatedMeshBuilder};
use crate::renderer::raytracer::raytracer_configuration::RayTracerConfiguration;
use crate::renderer::raytracer::rendering_threadpool::{RenderingThreadPool, ThreadPoolRenderer};
use crate::renderer::raytracer::shading::shade_hit_with_material;
use std::sync::Arc;

pub struct RayTracer {
    configuration: RayTracerConfiguration,
    rendering_thread_pool: RenderingThreadPool,
}

impl RayTracer {
    pub fn new(state: RendererState) -> Self {
        let p1 = Point::new(X::new(-0.5), Y::new(-0.5), Z::new(0.5));
        let p2 = Point::new(X::new(0.5), Y::new(-0.5), Z::new(0.5));
        let p3 = Point::new(X::new(0.5), Y::new(0.5), Z::new(0.5));
        let p4 = Point::new(X::new(-0.5), Y::new(0.5), Z::new(0.5));

        let mut scene = Scene::new();

        let green = scene.add_material(AmbientMaterialBuilder::new(MaterialColor::new(
            R::new(0.05),
            G::new(0.95),
            B::new(0.05),
            A::new(1.0),
        )));

        let red = scene.add_material(AmbientMaterialBuilder::new(MaterialColor::new(
            R::new(0.95),
            G::new(0.05),
            B::new(0.05),
            A::new(1.0),
        )));

        scene.add_object(
            TriangulatedMeshBuilder::new(green).add_triangle(Triangle::new([p1, p2, p3])),
        );

        scene.add_object(
            TriangulatedMeshBuilder::new(red).add_triangle(Triangle::new([p1, p3, p4])),
        );

        let configuration: RayTracerConfiguration = state.into();

        let camera = Camera::new(
            configuration.size().get_width(),
            configuration.size().get_height(),
            std::f64::consts::FRAC_PI_4,
        );

        let renderer = Arc::new(RaytracerRenderer { scene, camera });

        let rendering_thread_pool = RenderingThreadPool::new(32, renderer);

        Self {
            configuration,
            rendering_thread_pool,
        }
    }

    pub fn render_image(&self) -> Render {
        let mut render = Render::new(self.configuration.size().clone());

        let mut pixel_position = render.next();

        loop {
            let Some(position) = pixel_position else {
                break;
            };

            match self.rendering_thread_pool.add_pixel_to_render(position) {
                Ok(_) => {
                    pixel_position = render.next();
                    continue;
                }
                Err(_) => panic!("Failed to add pixel to render"),
            }
        }

        loop {
            match render.add_pixel(self.rendering_thread_pool.get_rendered_pixel().unwrap()) {
                RenderState::InProgress => continue,
                RenderState::Completed => break,
            }
        }

        render
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
