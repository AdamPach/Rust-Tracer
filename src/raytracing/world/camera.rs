use crate::core::configuration::{Height, Width};
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::matrix3x3::Matrix3x3;
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::core::render::{PixelX, PixelY};
use crate::raytracing::intersection::ray::Ray;

pub struct Camera {
    width: Width,
    height: Height,
    view_from: Point,
    fy: f64,
    camera_to_world: Matrix3x3,
}
impl Camera {
    pub fn new(width: Width, height: Height, fov: f64) -> Self {
        let from = Point::new(X::new(0.0), Y::new(0.0), Z::new(-1.0));
        let at = Point::new(X::new(0.0), Y::new(0.0), Z::new(0.0));
        let up = Vector3::new(X::new(0.0), Y::new(1.0), Z::new(0.0));

        let fy = (height / 2.0) / (fov / 2.0).tan();

        let z_c = (from - at).norm();

        let x_c = up.cross(&z_c).norm();

        let y_c = z_c.cross(&x_c).norm();

        Self {
            width,
            height,
            view_from: from,
            fy,
            camera_to_world: Matrix3x3::new([x_c, y_c, z_c]),
        }
    }

    pub fn generate_ray(&self, x: PixelX, y: PixelY) -> Ray {
        let origin = self.view_from;

        let direction = self.get_direction_vector(x, y).norm();

        Ray::new(origin, direction, 0.0001)
    }

    fn get_direction_vector(&self, x: PixelX, y: PixelY) -> Vector3 {
        let direction = Vector3::new(
            X::new(x - (self.width / 2.0)),
            Y::new(y - (self.height / 2.0)),
            Z::new(-self.fy),
        );

        self.camera_to_world.mul_by_vec3(&direction)
    }
}
