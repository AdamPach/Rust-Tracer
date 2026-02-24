use crate::core::render::{PixelPosition, RenderPixel};
use crate::rendering::shading::{TracingContext, trace_ray};
use crate::rendering::threadpool::Renderer;
use crate::raytracing::{Camera, Scene};
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct RaytracerRenderer {
    scene: ArcSwap<Scene>,
    camera: ArcSwap<Camera>,
}

impl RaytracerRenderer {
    pub fn new(scene: Scene, camera: Camera) -> Self {
        Self {
            scene: ArcSwap::from_pointee(scene),
            camera: ArcSwap::from_pointee(camera),
        }
    }

    pub fn set_camera(&self, camera: Camera) {
        self.camera.store(Arc::new(camera));
    }

    pub fn set_scene(&self, scene: Scene) {
        self.scene.store(Arc::new(scene));
    }
}

impl Renderer for RaytracerRenderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel {
        let (x, y) = position.get_pixel_coordinates();

        let ray = self.camera.load().generate_ray(x, y);

        let scene = self.scene.load();

        let ctx = TracingContext::new(&scene, 30);

        position.create_render_pixel(trace_ray(ray, ctx))
    }
}
