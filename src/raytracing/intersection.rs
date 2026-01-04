use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::geometry::{GeometryId, GeometryIndex, GeometryPrimitiveId};

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
    geometry_index: GeometryIndex,
}

impl RayHit {
    pub fn barycentric(&self) -> &Barycentric {
        &self.barycentric
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn geometry_index(&self) -> &GeometryIndex {
        &self.geometry_index
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
    geometry_primitive_id: GeometryPrimitiveId,
}

impl Hit {
    pub fn new(
        barycentric: Barycentric,
        ray: Ray,
        distance: f64,
        geometry_primitive_id: GeometryPrimitiveId,
    ) -> Self {
        Self {
            barycentric,
            ray,
            distance,
            geometry_primitive_id,
        }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn ray_hit(self, geometry_id: GeometryId) -> RayHit {
        RayHit {
            barycentric: self.barycentric,
            distance: self.distance,
            ray: self.ray,
            geometry_index: GeometryIndex::new(geometry_id, self.geometry_primitive_id),
        }
    }
}
