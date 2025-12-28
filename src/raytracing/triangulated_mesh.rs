use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::coordinates::{U, V};
use crate::core::geometry::point::Point;
use crate::raytracing::intersection::Hit;
use crate::raytracing::intersection::Ray;
use crate::raytracing::material::MaterialTypeId;
use crate::raytracing::scene::Geometry;

pub struct TriangulatedMesh {
    triangles: Vec<Triangle>,
}

impl TriangulatedMesh {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut intersection: Option<Hit> = None;

        for triangle in &self.triangles {
            let hit = match triangle.intersect(ray) {
                Some(hit) => hit,
                None => continue,
            };

            let Some(ray_hit) = intersection else {
                intersection = Some(hit);
                continue;
            };

            if ray_hit.distance() < hit.distance() {
                intersection = Some(ray_hit);
                continue;
            }

            intersection = Some(hit);
        }

        intersection
    }
}

#[derive(Debug)]
pub struct Triangle {
    points: [Point; 3],
    // normals: [Vector3; 3],
    material_id: MaterialTypeId,
}

impl Triangle {
    pub fn new(points: [Point; 3], material_id: MaterialTypeId) -> Self {
        Self {
            points,
            material_id,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];

        let p = ray.direction().cross(&e2);

        let mut det = e1.dot(&p);

        if det.abs() < 1e-12_f64 {
            return None;
        }

        det = 1.0 / det;

        let tvec = *ray.origin() - self.points[0];
        let u = U::new(tvec.dot(&p) * det);
        if u.get() < 0.0 || u.get() > 1.0 {
            return None;
        }

        let q = tvec.cross(&e1);
        let v = V::new(ray.direction().dot(&q) * det);
        if v.get() < 0.0 || u.get() + v.get() > 1.0 {
            return None;
        }

        let distance = e2.dot(&q) * det;

        if distance < ray.start_distance() {
            return None;
        }

        let barycentric = Barycentric::new(u, v);

        Some(Hit::new(barycentric, self.material_id, distance))
    }
}

#[derive(Debug)]
pub struct TriangulatedMeshBuilder {
    triangles: Vec<Triangle>,
}

impl TriangulatedMeshBuilder {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);

        self
    }

    pub fn build(self) -> TriangulatedMesh {
        TriangulatedMesh {
            triangles: self.triangles,
        }
    }
}

impl Into<Geometry> for TriangulatedMeshBuilder {
    fn into(self) -> Geometry {
        Geometry::TriangulatedMesh(self.build())
    }
}
