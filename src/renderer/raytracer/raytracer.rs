use crate::core::configuration::RendererState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::{ColorRGBA, Render, A, B, G, R};
use crate::raytracing::world::camera::Camera;
use crate::raytracing::world::scene::{Scene, SceneObject};
use crate::raytracing::world::triangulated_mesh::{Triangle, TriangulatedMesh};
use crate::renderer::raytracer::raytracer_configuration::RayTracerConfiguration;

pub struct RayTracer {
    configuration: RayTracerConfiguration,
    scene: Scene,
    camera: Camera,
}

impl RayTracer {
    pub fn new(state: RendererState) -> Self {
        let p1 = Point::new(X::new(-0.5), Y::new(-0.5), Z::new(0.5));
        let p2 = Point::new(X::new(0.5), Y::new(-0.5), Z::new(0.5));
        let p3 = Point::new(X::new(0.5), Y::new(0.5), Z::new(0.5));
        let p4 = Point::new(X::new(-0.5), Y::new(0.5), Z::new(0.5));

        let t1 = Triangle::new([p1, p2, p3]);
        let t2 = Triangle::new([p1, p3, p4]);

        let mut scene = Scene::new();

        scene.add_object(SceneObject::TriangulatedMesh(TriangulatedMesh::new(vec![
            t1, t2,
        ])));

        let configuration: RayTracerConfiguration = state.into();

        let camera = Camera::new(
            configuration.size().get_width(),
            configuration.size().get_height(),
            std::f64::consts::FRAC_PI_4,
        );

        Self {
            configuration,
            scene,
            camera,
        }
    }

    pub fn render_image(&self) -> Render {
        let mut render = Render::new(self.configuration.size().clone());

        while let Some(position) = render.next() {
            let pixel_coords = position.get_pixel_coordinates();

            let ray = self.camera.generate_ray(pixel_coords.0, pixel_coords.1);

            let rgba = if self.scene.find_intersection(ray).is_some() {
                ColorRGBA::new(R::new(255), G::new(0), B::new(0), A::new(255))
            } else {
                ColorRGBA::new(R::new(0), G::new(0), B::new(0), A::new(255))
            };

            render.add_pixel(position.create_render_pixel(rgba));
        }

        render
    }
}
