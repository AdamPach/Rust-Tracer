use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::material::MaterialTypeId;

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
}

pub struct Hit {
    barycentric: Barycentric,
    distance: f64,
    material_id: MaterialTypeId,
    normals: [Vector3; 3],
}

impl Hit {
    pub fn new(
        barycentric: Barycentric,
        material_id: MaterialTypeId,
        distance: f64,
        normals: [Vector3; 3],
    ) -> Self {
        Self {
            barycentric,
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
        }
    }
}
