use std::path::PathBuf;
use crate::io::wavefront::obj::load_obj;
use crate::raytracing::{Scene, SceneBuilder, SceneDescriptor};

pub struct WavefrontLoader {
    path: PathBuf,
}

impl WavefrontLoader {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        WavefrontLoader { path: path.into() }
    }

    fn load_wavefront(&self) -> anyhow::Result<SceneDescriptor> {

        let mut obj_file = self.path.clone();
        obj_file.add_extension("obj");
        
        Ok(load_obj(obj_file)?)
    }
}

impl SceneBuilder for WavefrontLoader {
    fn build_scene(&self) -> Scene {
        Scene::new(self.load_wavefront().unwrap())
    }
}
