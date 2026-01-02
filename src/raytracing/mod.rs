mod camera;
mod intersection;
pub mod material;
mod scene;
mod triangulated_mesh;

pub use camera::Camera;
pub use intersection::{Ray, RayHit};
pub use scene::{Scene, SceneBuilder, SceneDescriptor};
pub use triangulated_mesh::{Triangle, TriangulatedMeshBuilder};
