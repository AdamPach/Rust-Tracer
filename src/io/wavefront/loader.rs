use crate::io::wavefront::obj::load_obj;
use crate::raytracing::{Scene, SceneBuilder, SceneDescriptor};
use std::path::PathBuf;

pub struct WavefrontLoader {
    path: PathBuf,
}

impl WavefrontLoader {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        WavefrontLoader { path: path.into() }
    }

    fn load_wavefront(self) -> anyhow::Result<SceneDescriptor> {
        Ok(load_obj(self.path)?)
    }
}

impl SceneBuilder for WavefrontLoader {
    fn build_scene(self) -> Scene {
        Scene::new(self.load_wavefront().unwrap())
    }
}
