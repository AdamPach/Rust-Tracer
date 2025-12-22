use crate::core::render::{PixelPosition, RenderPixel};
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{Camera, Scene};
use crate::renderer::raytracer::shading::shade_hit_with_material;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};

pub struct RenderingThreadPool {
    threads: Vec<std::thread::JoinHandle<()>>,
    pixel_position_sender: Sender<PixelPosition>,
    render_pixel_receiver: Receiver<RenderPixel>,
}

impl RenderingThreadPool {
    pub fn new(number_of_threads: u16, scene: Arc<Scene>, camera: Arc<Camera>) -> Self {
        let mut threads = Vec::with_capacity(number_of_threads as usize);

        let (pixel_position_sender, pixel_position_receiver) =
            std::sync::mpsc::channel::<PixelPosition>();
        let (render_pixel_sender, render_pixel_receiver) =
            std::sync::mpsc::channel::<RenderPixel>();

        let pixel_position_receiver = Arc::new(Mutex::new(pixel_position_receiver));

        for _ in 0..number_of_threads {
            let scene = Arc::clone(&scene);
            let camera = Arc::clone(&camera);
            let pixel_position_receiver = pixel_position_receiver.clone();
            let render_pixel_sender = render_pixel_sender.clone();

            threads.push(spawn_thread(
                scene,
                camera,
                pixel_position_receiver,
                render_pixel_sender,
            ));
        }

        Self {
            threads,
            pixel_position_sender,
            render_pixel_receiver,
        }
    }

    pub fn add_pixel_to_render(&self, position: PixelPosition) -> Result<(), ()> {
        match self.pixel_position_sender.send(position) {
            Ok(_) => {
                Ok(())
            },
            Err(e) => match e {
                SendError(_) => Err(())
            },
        }
    }

    pub fn get_rendered_pixel(&self) -> Option<RenderPixel> {
        match self.render_pixel_receiver.recv() {
            Ok(pixel) => {
                Some(pixel)
            },
            Err(_) => None,
        }
    }
}

fn spawn_thread(
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    pixel_position_receiver: Arc<Mutex<Receiver<PixelPosition>>>,
    render_pixel_sender: Sender<RenderPixel>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        loop {
            let Ok(position) = pixel_position_receiver.lock().unwrap().recv() else {
                break;
            };

            let render_pixel = render_pixel(position, &scene, &camera);

            if let Err(_) = render_pixel_sender.send(render_pixel) {
                break;
            }
        }
    })
}

fn render_pixel(position: PixelPosition, scene: &Arc<Scene>, camera: &Arc<Camera>) -> RenderPixel {
    let (x, y) = position.get_pixel_coordinates();

    let ray = camera.generate_ray(x, y);

    let mut output_color =
        MaterialColor::new(R::new(0.05), G::new(0.05), B::new(0.05), A::new(1.0));

    if let Some(ray_hit) = scene.find_intersection(ray) {
        if let Some(material) = scene.get_material(ray_hit.material_id()) {
            output_color = shade_hit_with_material(material);
        }
    }

    position.create_render_pixel(output_color)
}
