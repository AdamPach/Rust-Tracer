use crate::raytracing::intersection::ray::Ray;
use crate::raytracing::intersection::ray_hit::RayHit;
use crate::raytracing::object::TriangulatedMesh;
use crate::raytracing::object::material::material_type::{MaterialType, MaterialTypeId};
use std::collections::HashMap;

pub struct Scene {
    objects: Vec<SceneObject>,
    materials: HashMap<MaterialTypeId, MaterialType>,
}

pub enum SceneObject {
    TriangulatedMesh(TriangulatedMesh),
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            objects: Vec::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
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
