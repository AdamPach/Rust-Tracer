use crate::core::render::Render;
use crate::renderer::raytracer::RayTracer;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct RenderingThread {
    render_receiver: Receiver<Render>,
}

impl RenderingThread {
    pub fn new(renderer: RayTracer) -> Self {
        let renderer_channel = mpsc::channel();

        rendering_thread(renderer, renderer_channel.0);

        RenderingThread {
            render_receiver: renderer_channel.1,
        }
    }

    pub fn try_recv_render(&mut self) -> Result<Render, TryRecvError> {
        match self.render_receiver.try_recv() {
            Ok(render) => {
                let mut render = render;

                while let Ok(new_render) = self.render_receiver.try_recv() {
                    render = new_render;
                }

                Ok(render)
            }
            Err(TryRecvError::Empty) => Err(TryRecvError::Empty),
            Err(TryRecvError::Disconnected) => panic!("render thread disconnected"),
        }
    }
}

fn rendering_thread(renderer: RayTracer, render_sender: Sender<Render>) {
    std::thread::spawn(move || {
        loop {
            let render = renderer.render_image();

            render_sender.send(render).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
