use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Matrix3x3 {
    data: [Vector3; 3],
}

impl Matrix3x3 {
    pub fn new(data: [Vector3; 3]) -> Matrix3x3 {
        Matrix3x3 { 
            data
        }
    }

    pub fn mul_by_vec3(&self, vec3: &Vector3) -> Vector3 {
        
        Vector3::new(
            X::new(self.data[0].dot(vec3)),
            Y::new(self.data[1].dot(vec3)),
            Z::new(self.data[2].dot(vec3)),
        )
    }
    
    pub fn transpose(self) -> Self {
        let row0 = Vector3::new(
            X::new(self.data[0].get_x().get()),
            Y::new(self.data[1].get_x().get()),
            Z::new(self.data[2].get_x().get()),
        );
        
        let row1 = Vector3::new(
            X::new(self.data[0].get_y().get()),
            Y::new(self.data[1].get_y().get()),
            Z::new(self.data[2].get_y().get()),
        );
        
        let row2 = Vector3::new(
            X::new(self.data[0].get_z().get()),
            Y::new(self.data[1].get_z().get()),
            Z::new(self.data[2].get_z().get()),
        );
        
        Matrix3x3::new([row0, row1, row2])
    }
}
