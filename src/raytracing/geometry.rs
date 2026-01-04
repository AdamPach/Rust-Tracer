use crate::core::geometry::barycentric::Barycentric;
use crate::core::geometry::vector::Vector3;
use crate::raytracing::Triangle;
use crate::raytracing::material::MaterialTypeId;
use crate::raytracing::triangulated_mesh::{TriangleId, TriangulatedMesh};
use std::collections::HashMap;

pub enum Geometry {
    TriangulatedMesh(TriangulatedMesh),
}

impl Geometry {
    pub fn primitive(&self, id: &GeometryPrimitiveId) -> GeometryPrimitive<'_> {
        match (self, id) {
            (Geometry::TriangulatedMesh(mesh), GeometryPrimitiveId::TriangleId(triangle_id)) => {
                GeometryPrimitive::Triangle(mesh.get_triangle(triangle_id))
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GeometryId(i32);

pub enum GeometryPrimitive<'a> {
    Triangle(&'a Triangle),
}

impl GeometryPrimitive<'_> {
    pub fn material_id(&self) -> MaterialTypeId {
        match self {
            GeometryPrimitive::Triangle(triangle) => triangle.material_id(),
        }
    }

    pub fn interpolate_normal(&self, barycentric_coords: &Barycentric) -> Vector3 {
        match self {
            GeometryPrimitive::Triangle(triangle) => {
                triangle.interpolate_normal(barycentric_coords)
            }
        }
    }
}

pub enum GeometryPrimitiveId {
    TriangleId(TriangleId),
}

pub struct GeometryIndex(GeometryId, GeometryPrimitiveId);

impl GeometryIndex {
    pub fn new(geometry_id: GeometryId, primitive_id: GeometryPrimitiveId) -> Self {
        Self(geometry_id, primitive_id)
    }
}

pub struct GeometryRegistry {
    next_id: i32,
    geometries: HashMap<GeometryId, Geometry>,
}

impl GeometryRegistry {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            geometries: HashMap::new(),
        }
    }

    pub fn add<T: Into<Geometry>>(&mut self, geometry: T) -> GeometryId {
        let id = GeometryId(self.next_id);
        self.geometries.insert(id, geometry.into());
        self.next_id += 1;
        id
    }

    pub fn get(&self, index: &GeometryIndex) -> GeometryPrimitive<'_> {
        let geometry = self.geometries.get(&index.0).unwrap();

        geometry.primitive(&index.1)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GeometryId, &Geometry)> {
        self.geometries.iter()
    }
}
