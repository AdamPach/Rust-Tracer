use crate::RendererConfiguration;
use crate::application::renderer::{RGBA, Render};
use rand::Rng;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct RayTracer {
    configuration: Arc<RwLock<RendererConfiguration>>,
    render_thread: Option<JoinHandle<()>>,
}

impl RayTracer {
    pub fn new(configuration: Arc<RwLock<RendererConfiguration>>) -> Self {
        Self {
            configuration,
            render_thread: None,
        }
    }

    pub fn start(&mut self, sender: Sender<Render>) {
        let configuration = self.configuration.clone();

        let handler = thread::spawn(move || {
            let configuration_lock = configuration.read().unwrap();
            let size = configuration_lock.size().clone();
            drop(configuration_lock);

            loop {
                let mut render = Render::new(size.clone());

                let mut rng = rand::rng();

                let r = rng.random::<u8>();
                let g = rng.random::<u8>();
                let b = rng.random::<u8>();

                for _h in 0..size.get_height() {
                    for _w in 0..size.get_width() {
                        let rgba = RGBA { r, g, b, a: 255 };

                        render.add_pixel(rgba);
                    }
                }

                if let Err(_e) = sender.send(render) {
                    break;
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        self.render_thread = Some(handler);
    }
}
