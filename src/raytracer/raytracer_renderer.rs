use crate::core::render::{PixelPosition, RenderPixel};
use crate::raytracer::threadpool::Renderer;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene};
use arc_swap::ArcSwap;
use std::sync::Arc;
use crate::raytracer::shading::shade_hit_with_material;

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

        let mut output_color =
            MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));

        let scene = self.scene.load();

        if let Some(ray_hit) = scene.find_intersection(ray) {
            if let Some(material) = scene.get_material(ray_hit.material_id()) {
                output_color = shade_hit_with_material(material, &scene);
            }
        }

        position.create_render_pixel(output_color)
    }
}
