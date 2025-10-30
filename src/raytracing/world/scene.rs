use crate::raytracing::intersection::ray::Ray;
use crate::raytracing::intersection::ray_hit::RayHit;
use crate::raytracing::world::triangulated_mesh::TriangulatedMesh;

pub struct Scene {
    objects: Vec<SceneObject>,
}

pub enum SceneObject {
    TriangulatedMesh(TriangulatedMesh),
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
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
