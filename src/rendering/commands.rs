use crate::io::wavefront::WavefrontLoader;
use crate::raytracing::SceneBuilder;
use crate::rendering::CameraSettings;
use std::path::PathBuf;

pub enum RaytracerCommand {
    CameraUpdate(CameraSettings),
    SceneUpdate(SceneLoadingDta),
    ClearAccumulator,
}

pub enum RaytracerResponse {
    SceneLoaded,
    CameraUpdated,
    AccumulatorCleared,
}

pub enum SceneLoadingDta {
    WavefrontObj { path: PathBuf },
}

impl SceneLoadingDta {
    pub fn loader(&self) -> impl SceneBuilder {
        match self {
            SceneLoadingDta::WavefrontObj { path } => WavefrontLoader::new(path),
        }
    }
}
