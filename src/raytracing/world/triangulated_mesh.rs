use crate::raytracing::geometry::barycentric::Barycentric;
use crate::raytracing::geometry::coordinates::{U, V};
use crate::raytracing::geometry::point::Point;
use crate::raytracing::intersection::hit::Hit;
use crate::raytracing::intersection::ray::Ray;
use crate::raytracing::intersection::ray_hit::RayHit;

pub struct Triangle {
    points: [Point; 3],
    // normals: [Vector3; 3],
}

pub struct TriangulatedMesh {
    triangles: Vec<Triangle>,
}

impl TriangulatedMesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<RayHit> {
        let mut intersection: Option<RayHit> = None;

        for triangle in &self.triangles {
            let hit = match triangle.intersect(ray) {
                Some(hit) => hit,
                None => continue,
            };

            let Some(ray_hit) = intersection else {
                intersection = Some(hit.into_ray_hit());
                continue;
            };

            if ray_hit.distance() < hit.distance() {
                intersection = Some(ray_hit);
                continue;
            }

            intersection = Some(hit.into_ray_hit());
        }

        intersection
    }
}

impl Triangle {
    pub fn new(points: [Point; 3] /*, normals: [Vector3; 3]*/) -> Self {
        Self {
            points, /*normals*/
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

        Some(Hit::new(barycentric, distance))
    }
}
