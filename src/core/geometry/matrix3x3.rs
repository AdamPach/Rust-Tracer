use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::vector::Vector3;

pub struct Matrix3x3 {
    data: [Vector3; 3],
}

impl Matrix3x3 {
    pub fn new(data: [Vector3; 3]) -> Matrix3x3 {
        Matrix3x3 { data }
    }

    pub fn mul_by_vec3(&self, vec3: &Vector3) -> Vector3 {
        Vector3::new(
            X::new(self.data[0].dot(vec3)),
            Y::new(self.data[1].dot(vec3)),
            Z::new(self.data[2].dot(vec3)),
        )
    }
}
