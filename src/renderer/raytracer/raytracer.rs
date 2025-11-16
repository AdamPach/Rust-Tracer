use crate::core::configuration::RendererState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::Render;
use crate::raytracing::object::material::ambient::Ambient;
use crate::raytracing::object::material::color::{A, B, Color, G, R};
use crate::raytracing::object::shader::Shader;
use crate::raytracing::object::{TriangleData, TriangulatedMeshBuilder};
use crate::raytracing::world::{Camera, Scene, SceneObject};
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

        let mut scene = Scene::new();

        let material = scene.add_material(Ambient::new(Color::new(
            R::new(0.05),
            G::new(0.95),
            B::new(0.05),
            A::new(1.0),
        )));

        let mesh_builder = TriangulatedMeshBuilder::new(material)
            .add_triangle(TriangleData::new([p1, p2, p3]))
            .add_triangle(TriangleData::new([p1, p3, p4]));

        scene.add_object(SceneObject::TriangulatedMesh(mesh_builder.build()));

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

            let color = match self.scene.find_intersection(ray) {
                Some(hit) => self
                    .scene
                    .get_material(hit.material_id())
                    .unwrap()
                    .shader()
                    .shade(hit),
                None => Color::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0)),
            };

            render.add_pixel(position.create_render_pixel(color));
        }

        render
    }
}
