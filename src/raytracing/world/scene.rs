use crate::raytracing::intersection::Ray;
use crate::raytracing::intersection::RayHit;
use crate::raytracing::object::TriangulatedMesh;
use crate::raytracing::shading::{Material, MaterialId};
use std::collections::HashMap;

pub struct Scene {
    objects: Vec<SceneObject>,
    materials: HashMap<MaterialId, Material>,
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

    pub fn add_material(&mut self, material: impl Into<Material>) -> MaterialId {
        let id = MaterialId::new(self.objects.len() as i32);
        self.materials.insert(id, material.into());
        id
    }

    pub fn get_material(&self, material: MaterialId) -> Option<&Material> {
        self.materials.get(&material)
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

        for object in &self.objects {
            let ray_hit = match object {
                SceneObject::TriangulatedMesh(m) => match m.intersect(&ray) {
                    Some(hit) => hit,
                    None => continue,
                },
            };

            let Some(nearest_ray_hit) = intersection else {
                intersection = Some(ray_hit);
                continue;
            };

            if nearest_ray_hit.distance() < ray_hit.distance() {
                intersection = Some(nearest_ray_hit);
                continue;
            }

            intersection = Some(ray_hit);
        }

        intersection
    }
}
