use crate::core::configuration::RendererState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::{Render};
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene, Triangle, TriangulatedMeshBuilder};
use crate::renderer::raytracer::raytracer_configuration::RayTracerConfiguration;
use crate::renderer::raytracer::rendering_threadpool::{RenderingThreadPool};
use std::sync::Arc;

pub struct RayTracer {
    configuration: RayTracerConfiguration,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
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

        let scene = Arc::new(scene);
        let camera = Arc::new(camera);

        let rendering_thread_pool = RenderingThreadPool::new(32, scene.clone(), camera.clone());

        Self {
            configuration,
            scene,
            camera,
            rendering_thread_pool,
        }
    }

    pub fn render_image(&self) -> Render {
        let mut render = Render::new(self.configuration.size().clone());

        let mut pixel_position = render.next();

        let mut rendering_count = 0u32;

        loop {
            let Some(position) = pixel_position else {
                break;
            };

            match self.rendering_thread_pool.add_pixel_to_render(position) {
                Ok(_) => {
                    pixel_position = render.next();
                    rendering_count += 1;
                    continue;
                }
                Err(_) => panic!("Failed to add pixel to render"),
            }
        }

        while rendering_count > 0 {
            render.add_pixel(self.rendering_thread_pool.get_rendered_pixel().unwrap());
            rendering_count -= 1;
        }

        render
    }
}
