mod camera;
mod scene;
mod intersection;
mod triangulated_mesh;
pub mod material;

pub use intersection::RayHit;
pub use camera::Camera;
pub use scene::Scene;
pub use triangulated_mesh::{Triangle, TriangulatedMeshBuilder};
