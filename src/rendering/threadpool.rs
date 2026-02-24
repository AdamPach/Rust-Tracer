use crate::core::render::{AccumulatedRender, PixelPosition, RenderIterator, RenderPixel};
use std::ops::Deref;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};

pub trait Renderer {
    fn render_pixel(&self, position: PixelPosition) -> RenderPixel;
}

pub struct ThreadPool<T> {
    render_pixel_receiver: Receiver<RenderPixel>,
    render_iterator_sender: SyncSender<RenderIterator>,
    renderer: Arc<T>,
}

impl<T> ThreadPool<T>
where
    T: Renderer,
    T: Send + Sync + 'static,
{
    pub fn new(number_of_threads: u16, renderer: T) -> Self {
        let (pixel_position_sender, pixel_position_receiver) =
            std::sync::mpsc::sync_channel::<PixelPosition>((number_of_threads * 2) as usize);

        let (render_pixel_sender, render_pixel_receiver) =
            std::sync::mpsc::channel::<RenderPixel>();

        let (render_iterator_sender, render_iterator_receiver) =
            std::sync::mpsc::sync_channel::<RenderIterator>(0);

        let pixel_position_receiver = Arc::new(Mutex::new(pixel_position_receiver));

        let renderer = Arc::new(renderer);

        for _ in 0..number_of_threads {
            let pixel_position_receiver = pixel_position_receiver.clone();
            let render_pixel_sender = render_pixel_sender.clone();
            let renderer = renderer.clone();

            spawn_rendering_thread(renderer, pixel_position_receiver, render_pixel_sender);
        }

        spawn_iterator_thread(render_iterator_receiver, pixel_position_sender);

        Self {
            render_pixel_receiver,
            render_iterator_sender,
            renderer,
        }
    }

    pub fn render<'a>(&self, mut render: AccumulatedRender<'a>) -> AccumulatedRender<'a> {
        let _ = self.render_iterator_sender.send(render.iterator());

        loop {
            match self.render_pixel_receiver.recv() {
                Ok(pixel) => match render.add_pixel(pixel) {
                    crate::core::render::RenderState::InProgress => continue,
                    crate::core::render::RenderState::Completed => {
                        break;
                    }
                },
                Err(_) => break,
            }
        }

        render
    }
}

impl<T> Deref for ThreadPool<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

fn spawn_iterator_thread(
    render_iterator_receiver: Receiver<RenderIterator>,
    pixel_position_sender: SyncSender<PixelPosition>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        loop {
            let Ok(mut iterator) = render_iterator_receiver.recv() else {
                break;
            };

            let mut pixel_position = iterator.next();

            loop {
                let Some(position) = pixel_position else {
                    break;
                };

                let _ = pixel_position_sender
                    .send(position)
                    .expect("Failed to send pixel position to rendering threads");

                pixel_position = iterator.next();
            }
        }
    })
}

fn spawn_rendering_thread<T>(
    renderer: Arc<T>,
    pixel_position_receiver: Arc<Mutex<Receiver<PixelPosition>>>,
    render_pixel_sender: Sender<RenderPixel>,
) -> std::thread::JoinHandle<()>
where
    T: Renderer,
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
