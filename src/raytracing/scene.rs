use crate::raytracing::intersection::Ray;
use crate::raytracing::intersection::RayHit;
use crate::raytracing::material::{MaterialType, MaterialTypeId, MaterialsRegistry};
use crate::raytracing::triangulated_mesh::TriangulatedMesh;

pub trait SceneBuilder {
    fn build_scene(self) -> anyhow::Result<Scene>;
}

#[derive(Clone)]
pub enum Geometry {
    TriangulatedMesh(TriangulatedMesh),
}

#[derive(Clone)]
pub struct Scene {
    scene_descriptor: SceneDescriptor,
}

impl Scene {
    pub fn get_material(&self, material: MaterialTypeId) -> Option<&MaterialType> {
        self.scene_descriptor.materials.get(material)
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

        for object in &self.scene_descriptor.objects {
            let hit = match object {
                Geometry::TriangulatedMesh(m) => match m.intersect(&ray) {
                    Some(hit) => hit,
                    None => continue,
                },
            };

            let Some(nearest_ray_hit) = intersection else {
                intersection = Some(hit.ray_hit());
                continue;
            };

            if nearest_ray_hit.distance() < hit.distance() {
                intersection = Some(nearest_ray_hit);
                continue;
            }

            intersection = Some(hit.ray_hit());
        }

        intersection
    }
}

#[derive(Clone)]
pub struct SceneDescriptor {
    objects: Vec<Geometry>,
    materials: MaterialsRegistry,
}

impl SceneDescriptor {
    pub fn new(materials: MaterialsRegistry) -> Self {
        Self {
            objects: Vec::new(),
            materials,
        }
    }

    pub fn add_object<T: Into<Geometry>>(&mut self, object: T) {
        self.objects.push(object.into());
    }

    pub fn scene(self) -> Scene {
        Scene {
            scene_descriptor: self,
        }
    }
}

impl Default for SceneDescriptor {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            materials: MaterialsRegistry::new(),
        }
    }
}
