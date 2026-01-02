use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Matrix3x3 {
    data: [Vector3; 3],
}

impl Matrix3x3 {
    pub fn new(data: [Vector3; 3]) -> Matrix3x3 {
        Matrix3x3 { data }
    }

    pub fn from_columns(col0: Vector3, col1: Vector3, col2: Vector3) -> Matrix3x3 {
        Matrix3x3 {
            data: [
                Vector3::new(
                    X::new(col0.get_x().get()),
                    Y::new(col1.get_x().get()),
                    Z::new(col2.get_x().get()),
                ),
                Vector3::new(
                    X::new(col0.get_y().get()),
                    Y::new(col1.get_y().get()),
                    Z::new(col2.get_y().get()),
                ),
                Vector3::new(
                    X::new(col0.get_z().get()),
                    Y::new(col1.get_z().get()),
                    Z::new(col2.get_z().get()),
                ),
            ],
        }
    }

    pub fn mul_by_vec3(&self, vec3: &Vector3) -> Vector3 {
        Vector3::new(
            X::new(self.data[0].dot(vec3)),
            Y::new(self.data[1].dot(vec3)),
            Z::new(self.data[2].dot(vec3)),
        )
    }
}
