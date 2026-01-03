use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::material::MaterialTypeId;

#[derive(Clone)]
pub struct Ray {
    origin: Point,
    direction: Vector3,
    start_distance: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3, start_distance: f64) -> Self {
        Self {
            origin,
            direction,
            start_distance,
        }
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn start_distance(&self) -> f64 {
        self.start_distance
    }
}

pub struct RayHit {
    barycentric: Barycentric,
    ray: Ray,
    distance: f64,
    material_id: MaterialTypeId,
    normals: [Vector3; 3],
}

impl RayHit {
    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn material_id(&self) -> MaterialTypeId {
        self.material_id
    }

    pub fn interpolated_normal(&self) -> Vector3 {
        let u = self.barycentric.u().get();
        let v = self.barycentric.v().get();
        let w = 1.0 - u - v;

        (self.normals[0] * w + self.normals[1] * u + self.normals[2] * v).norm()
    }

    pub fn hit_point(&self) -> Point {
        Point::new(
            X::new(self.ray.origin().x().get() + self.ray.direction().x().get() * self.distance),
            Y::new(self.ray.origin().y().get() + self.ray.direction().y().get() * self.distance),
            Z::new(self.ray.origin().z().get() + self.ray.direction().z().get() * self.distance),
        )
    }
}

pub struct Hit {
    barycentric: Barycentric,
    ray: Ray,
    distance: f64,
    material_id: MaterialTypeId,
    normals: [Vector3; 3],
}

impl Hit {
    pub fn new(
        barycentric: Barycentric,
        ray: Ray,
        material_id: MaterialTypeId,
        distance: f64,
        normals: [Vector3; 3],
    ) -> Self {
        Self {
            barycentric,
            ray,
            distance,
            material_id,
            normals,
        }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn ray_hit(self) -> RayHit {
        RayHit {
            barycentric: self.barycentric,
            distance: self.distance,
            material_id: self.material_id,
            normals: self.normals,
            ray: self.ray,
        }
    }
}
