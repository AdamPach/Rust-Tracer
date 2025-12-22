mod camera;
mod intersection;
pub mod material;
mod scene;
mod triangulated_mesh;

pub use camera::Camera;
pub use intersection::RayHit;
pub use scene::Scene;
pub use triangulated_mesh::{Triangle, TriangulatedMeshBuilder};
