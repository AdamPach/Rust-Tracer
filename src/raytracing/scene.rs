use crate::raytracing::intersection::Ray;
use crate::raytracing::intersection::RayHit;
use std::collections::HashMap;
use crate::raytracing::material::{MaterialType, MaterialTypeId};
use crate::raytracing::triangulated_mesh::TriangulatedMesh;

pub enum SceneObject {
    TriangulatedMesh(TriangulatedMesh),
}

pub struct Scene {
    objects: Vec<SceneObject>,
    materials: HashMap<MaterialTypeId, MaterialType>,
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            objects: Vec::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_object<T: Into<SceneObject>>(&mut self, object: T) {
        self.objects.push(object.into());
    }

    pub fn add_material(&mut self, material: impl Into<MaterialType>) -> MaterialTypeId {
        let id = MaterialTypeId::new(self.materials.len() as i32);
        self.materials.insert(id, material.into());
        id
    }

    pub fn get_material(&self, material: MaterialTypeId) -> Option<&MaterialType> {
        self.materials.get(&material)
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

        for object in &self.objects {
            let hit = match object {
                SceneObject::TriangulatedMesh(m) => match m.intersect(&ray) {
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
