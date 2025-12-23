use crate::core::render::{PixelPosition, Render, RenderPixel};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};

pub trait ThreadPoolRenderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel;
}

pub struct RenderingThreadPool {
    rendering_threads: Vec<std::thread::JoinHandle<()>>,
    sync_thread: std::thread::JoinHandle<()>,
    pixel_position_sender: SyncSender<PixelPosition>,
    sync_render_sender: SyncSender<(Render, SyncSender<Render>)>,
}

impl RenderingThreadPool {
    pub fn new<T>(number_of_threads: u16, renderer: Arc<T>) -> Self
    where
        T: ThreadPoolRenderer,
        T: Send + Sync + 'static,
    {
        let mut rendering_threads = Vec::with_capacity(number_of_threads as usize);

        let (pixel_position_sender, pixel_position_receiver) =
            std::sync::mpsc::sync_channel::<PixelPosition>((number_of_threads * 2) as usize);

        let (render_pixel_sender, render_pixel_receiver) =
            std::sync::mpsc::channel::<RenderPixel>();

        let (sync_render_sender, sync_render_receiver) =
            std::sync::mpsc::sync_channel::<(Render, SyncSender<Render>)>(0);

        let pixel_position_receiver = Arc::new(Mutex::new(pixel_position_receiver));

        for _ in 0..number_of_threads {
            let pixel_position_receiver = pixel_position_receiver.clone();
            let render_pixel_sender = render_pixel_sender.clone();
            let renderer = renderer.clone();

            rendering_threads.push(spawn_rendering_thread(
                renderer,
                pixel_position_receiver,
                render_pixel_sender,
            ));
        }

        let sync_thread = spawn_sync_thread(sync_render_receiver, render_pixel_receiver);

        Self {
            rendering_threads,
            sync_thread,
            pixel_position_sender,
            sync_render_sender,
        }
    }

    pub fn render(&self, mut render: Render) -> Render {
        let (reply_sender, reply_receiver) = std::sync::mpsc::sync_channel::<Render>(0);

        let _ = self.sync_render_sender.send((render.clone(), reply_sender));

        let mut pixel_position = render.next();

        loop {
            let Some(position) = pixel_position else {
                break;
            };

            let _ = self
                .pixel_position_sender
                .send(position)
                .expect("Failed to send pixel position to rendering threads");

            pixel_position = render.next();
        }

        reply_receiver
            .recv()
            .expect("Failed to receive reply from rendering thread")
    }
}

fn spawn_sync_thread(
    sync_render_receiver: Receiver<(Render, SyncSender<Render>)>,
    render_pixel_receiver: Receiver<RenderPixel>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        loop {
            let Ok(sync) = sync_render_receiver.recv() else {
                break;
            };

            let (mut render, reply_sender) = sync;

            loop {
                match render_pixel_receiver.recv() {
                    Ok(pixel) => match render.add_pixel(pixel) {
                        crate::core::render::RenderState::InProgress => continue,
                        crate::core::render::RenderState::Completed => {
                            let _ = reply_sender.send(render);
                            break;
                        }
                    },
                    Err(_) => break,
                }
            }
        }
    })
}

fn spawn_rendering_thread<T: ThreadPoolRenderer>(
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
