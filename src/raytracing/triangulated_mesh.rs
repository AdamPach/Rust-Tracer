use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::coordinates::{U, V};
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::geometry::{Geometry, GeometryPrimitiveId};
use crate::raytracing::intersection::Hit;
use crate::raytracing::intersection::Ray;
use crate::raytracing::material::MaterialTypeId;

#[derive(Clone)]
pub struct TriangulatedMesh {
    triangles: Vec<Triangle>,
}

impl TriangulatedMesh {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut intersection: Option<Hit> = None;

        for (id, triangle) in self.triangles.iter().enumerate() {
            let hit = match triangle.intersect(ray, TriangleId(id)) {
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

    pub fn get_triangle(&self, id: &TriangleId) -> &Triangle {
        &self.triangles[id.0]
    }
}

pub struct TriangleId(usize);

#[derive(Debug, Clone)]
pub struct Triangle {
    points: [Point; 3],
    normals: [Vector3; 3],
    material_id: MaterialTypeId,
}

impl Triangle {
    pub fn new(points: [Point; 3], normals: [Vector3; 3], material_id: MaterialTypeId) -> Self {
        Self {
            points,
            material_id,
            normals,
        }
    }

    pub fn intersect(&self, ray: &Ray, id: TriangleId) -> Option<Hit> {
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

        Some(Hit::new(
            barycentric,
            ray.clone(),
            distance,
            GeometryPrimitiveId::TriangleId(id),
        ))
    }

    pub fn material_id(&self) -> MaterialTypeId {
        self.material_id
    }
    
    pub fn interpolate_normal(&self, barycentric_coords: &Barycentric) -> Vector3 {
        let u = barycentric_coords.u().get();
        let v = barycentric_coords.v().get();
        let w = 1.0 - u - v;

        self.normals[0] * w + self.normals[1] * u + self.normals[2] * v
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
