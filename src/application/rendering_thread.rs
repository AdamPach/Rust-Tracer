use crate::application::renderer::{Render, RendererCommand};
use crate::raytracer::RayTracer;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SendError, Sender, TryRecvError};

pub struct RenderingThread {
    render_receiver: Receiver<Render>,
    command_sender: Sender<RendererCommand>,
}

impl RenderingThread {
    pub fn new(renderer: RayTracer) -> Self {
        let renderer_channel = mpsc::channel();
        let command_channel = mpsc::channel();

        rendering_thread(renderer, renderer_channel.0, command_channel.1);

        RenderingThread {
            render_receiver: renderer_channel.1,
            command_sender: command_channel.0,
        }
    }

    pub fn try_recv_render(&mut self) -> Result<Render, TryRecvError> {
        self.render_receiver.try_recv()
    }

    pub fn send_command(
        &mut self,
        command: RendererCommand,
    ) -> Result<(), SendError<RendererCommand>> {
        self.command_sender.send(command)
    }
}

fn rendering_thread(
    renderer: RayTracer,
    render_sender: Sender<Render>,
    command_receiver: Receiver<RendererCommand>,
) {
    std::thread::spawn(move || {
        loop {
            let render = renderer.render_image();

            render_sender.send(render).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
