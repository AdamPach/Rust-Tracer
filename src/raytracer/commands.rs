use crate::core::render::Render;
use crate::io::wavefront::WavefrontLoader;
use crate::raytracer::CameraSettings;
use crate::raytracing::SceneBuilder;
use std::path::PathBuf;

pub enum RaytracerCommand {
    RenderFrame,
    CameraUpdate(CameraSettings),
    SceneUpdate(SceneLoadingDta),
}

pub enum RaytracerResponse {
    RenderComplete(Render),
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
