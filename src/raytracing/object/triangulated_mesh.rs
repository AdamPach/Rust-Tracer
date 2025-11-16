use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::coordinates::{U, V};
use crate::core::geometry::point::Point;
use crate::raytracing::intersection::Ray;
use crate::raytracing::intersection::RayHit;
use crate::raytracing::shading::MaterialId;

pub struct TriangulatedMesh {
    triangles: Vec<Triangle>,
}

impl TriangulatedMesh {
    pub fn intersect(&self, ray: &Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

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

struct Triangle {
    points: [Point; 3],
    material_id: MaterialId,
    // normals: [Vector3; 3],
}

impl Triangle {
    pub fn intersect(&self, ray: &Ray) -> Option<RayHit> {
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

        Some(RayHit::new(barycentric, self.material_id, distance))
    }
}

pub struct TriangleData {
    points: [Point; 3],
}

impl TriangleData {
    pub fn new(points: [Point; 3]) -> Self {
        Self { points }
    }
}

pub struct TriangulatedMeshBuilder {
    triangles: Vec<Triangle>,
    material_id: MaterialId,
}

impl TriangulatedMeshBuilder {
    pub fn new(material_id: MaterialId) -> Self {
        Self {
            material_id,
            triangles: Vec::new(),
        }
    }

    pub fn add_triangle(mut self, triangle: TriangleData) -> Self {
        self.triangles.push(Triangle {
            points: triangle.points,
            material_id: self.material_id,
        });

        self
    }

    pub fn build(self) -> TriangulatedMesh {
        TriangulatedMesh {
            triangles: self.triangles,
        }
    }
}
