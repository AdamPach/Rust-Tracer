use crate::RendererConfiguration;
use crate::application::renderer::{RGBA, Render};
use crate::raytracing::geometry::coordinates::{X, Y, Z};
use crate::raytracing::geometry::point::Point;
use crate::raytracing::world::camera::Camera;
use crate::raytracing::world::scene::{Scene, SceneObject};
use crate::raytracing::world::triangulated_mesh::{Triangle, TriangulatedMesh};

pub struct RayTracer {
    configuration: RendererConfiguration,
    scene: Scene,
    camera: Camera,
}

impl RayTracer {
    pub fn new(configuration: RendererConfiguration) -> Self {
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
        let size = self.configuration.size().clone();

        let mut render = Render::new(size.clone());

        for _h in 0..size.get_height() {
            for _w in 0..size.get_width() {
                let ray = self.camera.generate_ray(_w as f64, _h as f64);

                let rgba = if self.scene.find_intersection(ray).is_some() {
                    RGBA {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 255,
                    }
                } else {
                    RGBA {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    }
                };

                render.add_pixel(rgba);
            }
        }

        render
    }
}
