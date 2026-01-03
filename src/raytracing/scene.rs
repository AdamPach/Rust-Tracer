use crate::raytracing::geometry::{Geometry, GeometryIndex, GeometryPrimitive, GeometryRegistry};
use crate::raytracing::intersection::Ray;
use crate::raytracing::intersection::RayHit;
use crate::raytracing::material::{MaterialType, MaterialTypeId, MaterialsRegistry};

pub trait SceneBuilder {
    fn build_scene(self) -> anyhow::Result<Scene>;
}

pub struct Scene {
    scene_descriptor: SceneDescriptor,
}

impl Scene {
    pub fn get_material(&self, material: MaterialTypeId) -> Option<&MaterialType> {
        self.scene_descriptor.materials.get(material)
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

        for (id, geometry) in self.scene_descriptor.geometry.iter() {
            let hit = match geometry {
                Geometry::TriangulatedMesh(m) => match m.intersect(&ray) {
                    Some(hit) => hit,
                    None => continue,
                },
            };

            let Some(nearest_ray_hit) = intersection else {
                intersection = Some(hit.ray_hit(id.clone()));
                continue;
            };

            if nearest_ray_hit.distance() < hit.distance() {
                intersection = Some(nearest_ray_hit);
                continue;
            }

            intersection = Some(hit.ray_hit(id.clone()));
        }

        intersection
    }

    pub fn get_geometry(&self, index: &GeometryIndex) -> GeometryPrimitive<'_> {
        self.scene_descriptor.geometry.get(index)
    }
}

pub struct SceneDescriptor {
    geometry: GeometryRegistry,
    materials: MaterialsRegistry,
}

impl SceneDescriptor {
    pub fn new(geometry: GeometryRegistry, materials: MaterialsRegistry) -> Self {
        Self {
            geometry,
            materials,
        }
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
            geometry: GeometryRegistry::new(),
            materials: MaterialsRegistry::new(),
        }
    }
}
