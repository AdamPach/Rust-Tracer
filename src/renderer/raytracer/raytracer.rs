use crate::core::configuration::RendererState;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::render::{PixelX, PixelY, Render};
use crate::raytracing::{Camera, Scene, Triangle, TriangulatedMeshBuilder};
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{MaterialColor, A, B, G, R};
use crate::renderer::raytracer::raytracer_configuration::RayTracerConfiguration;
use crate::renderer::raytracer::shading::{shade_hit_with_material};

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
        
        scene.add_object(TriangulatedMeshBuilder::new(green).add_triangle(Triangle::new([p1, p2, p3])));
        
        scene.add_object(TriangulatedMeshBuilder::new(red).add_triangle(Triangle::new([p1, p3, p4])));

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
            let (x, y) = position.get_pixel_coordinates();

            render.add_pixel(position.create_render_pixel(self.render_pixel(x, y)));
        }

        render
    }

    fn render_pixel(&self, x: PixelX, y: PixelY) -> MaterialColor {
        let ray = self.camera.generate_ray(x, y);

        if let Some(ray_hit) = self.scene.find_intersection(ray) {

            if let Some(material) = self.scene.get_material(ray_hit.material_id()) {
                return shade_hit_with_material(material);
            }
        }

        MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0))
    }
}
