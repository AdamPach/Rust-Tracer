use crate::io::wavefront::obj::load_obj;
use crate::raytracing::{Scene, SceneBuilder};
use std::path::PathBuf;

pub struct WavefrontLoader {
    path: PathBuf,
}

impl WavefrontLoader {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        WavefrontLoader { path: path.into() }
    }
}

impl SceneBuilder for WavefrontLoader {
    fn build_scene(self) -> anyhow::Result<Scene> {
        let scene = load_obj(self.path)?;
        
        Ok(scene.to_scene())
    }
}
