use std::path::PathBuf;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::io::wavefront::obj::load_obj;
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Scene, SceneBuilder, SceneDescriptor, Triangle, TriangulatedMeshBuilder};

pub struct WavefrontLoader {
    path: PathBuf,
}

impl WavefrontLoader {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        WavefrontLoader { path: path.into() }
    }

    fn load_wavefront(&self) -> anyhow::Result<SceneDescriptor> {
        let mut scene = SceneDescriptor::new();

        let mut obj_file = self.path.clone();
        obj_file.add_extension("obj");

        load_obj(obj_file)?;

        let p1 = Point::new(X::new(-0.5), Y::new(-0.5), Z::new(0.5));
        let p2 = Point::new(X::new(0.5), Y::new(-0.5), Z::new(0.5));
        let p3 = Point::new(X::new(0.5), Y::new(0.5), Z::new(0.5));
        let p4 = Point::new(X::new(-0.5), Y::new(0.5), Z::new(0.5));

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

        Ok(scene)
    }
}

impl SceneBuilder for WavefrontLoader {
    fn build_scene(&self) -> Scene {
        Scene::new(self.load_wavefront().unwrap())
    }
}
