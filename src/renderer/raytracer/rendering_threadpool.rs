use crate::core::render::{PixelPosition, RenderPixel};
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};

pub trait ThreadPoolRenderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel;
}

pub struct RenderingThreadPool {
    threads: Vec<std::thread::JoinHandle<()>>,
    pixel_position_sender: Sender<PixelPosition>,
    render_pixel_receiver: Receiver<RenderPixel>,
}

impl RenderingThreadPool {
    pub fn new<T>(number_of_threads: u16, renderer: Arc<T>) -> Self
    where
        T: ThreadPoolRenderer,
        T: Send + Sync + 'static,
    {
        let mut threads = Vec::with_capacity(number_of_threads as usize);

        let (pixel_position_sender, pixel_position_receiver) =
            std::sync::mpsc::channel::<PixelPosition>();
        let (render_pixel_sender, render_pixel_receiver) =
            std::sync::mpsc::channel::<RenderPixel>();

        let pixel_position_receiver = Arc::new(Mutex::new(pixel_position_receiver));

        for _ in 0..number_of_threads {
            let pixel_position_receiver = pixel_position_receiver.clone();
            let render_pixel_sender = render_pixel_sender.clone();

            let renderer = Arc::clone(&renderer);

            threads.push(spawn_thread(
                renderer,
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
            Ok(_) => Ok(()),
            Err(e) => match e {
                SendError(_) => Err(()),
            },
        }
    }

    pub fn get_rendered_pixel(&self) -> Option<RenderPixel> {
        match self.render_pixel_receiver.recv() {
            Ok(pixel) => Some(pixel),
            Err(_) => None,
        }
    }
}

fn spawn_thread<T: ThreadPoolRenderer>(
    renderer: Arc<T>,
    pixel_position_receiver: Arc<Mutex<Receiver<PixelPosition>>>,
    render_pixel_sender: Sender<RenderPixel>,
) -> std::thread::JoinHandle<()>
where
    T: ThreadPoolRenderer,
    T: Send + Sync + 'static,
{
    std::thread::spawn(move || {
        loop {
            let Ok(position) = pixel_position_receiver.lock().unwrap().recv() else {
                break;
            };

            let render_pixel = renderer.render_pixel(position);

            if let Err(_) = render_pixel_sender.send(render_pixel) {
                break;
            }
        }
    })
}
