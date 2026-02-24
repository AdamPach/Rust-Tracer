use crate::core::render::Render;
use crate::io::wavefront::WavefrontLoader;
use crate::raytracing::SceneBuilder;
use crate::rendering::CameraSettings;
use std::path::PathBuf;

pub enum RaytracerCommand {
    RenderFrame,
    CameraUpdate(CameraSettings),
    SceneUpdate(SceneLoadingDta),
}

pub enum RaytracerResponse {
    RenderComplete(Render),
    SceneLoaded,
    RendererUpdated,
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
