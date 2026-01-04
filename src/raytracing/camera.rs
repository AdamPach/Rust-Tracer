use crate::core::configuration::Size;
use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::matrix3x3::Matrix3x3;
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::core::render::{PixelX, PixelY};
use crate::raytracer::CameraSettings;
use crate::raytracing::intersection::Ray;

pub struct Camera {
    size: Size,
    view_from: Point,
    fy: f64,
    camera_to_world: Matrix3x3,
}

impl Camera {
    pub fn new(size: Size, camera_settings: CameraSettings) -> Self {
        let up = Vector3::new(X::new(0.0), Y::new(1.0), Z::new(0.0));

        let fy = (size.get_height() / 2.0) / (camera_settings.fov / 2.0).tan();

        let z_c = (camera_settings.position - camera_settings.view_at).norm();

        let x_c = up.cross(&z_c).norm();

        let y_c = z_c.cross(&x_c).norm();

        let camera_to_world = Matrix3x3::from_columns(x_c, y_c, z_c);

        Self {
            size,
            view_from: camera_settings.position,
            fy,
            camera_to_world,
        }
    }

    pub fn generate_ray(&self, x: PixelX, y: PixelY) -> Ray {
        let origin = self.view_from;

        let direction = self.get_direction_vector(x, y).norm();

        Ray::new(origin, direction, 0.0001)
    }

    fn get_direction_vector(&self, x: PixelX, y: PixelY) -> Vector3 {
        let direction = Vector3::new(
            X::new(x - (self.size.get_width() / 2.0)),
            Y::new((self.size.get_height() / 2.0) - y.get()),
            Z::new(-self.fy),
        );

        self.camera_to_world.mul_by_vec3(&direction)
    }
}
