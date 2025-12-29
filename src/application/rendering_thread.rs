use crate::core::render::Render;
use crate::raytracer::{Raytracer, RaytracerCommand, RaytracerResponse};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};

pub struct RenderingThread {
    render_receiver: Receiver<Render>,
    renderer_command_sender: Sender<RaytracerCommand>,
}

impl RenderingThread {
    pub fn new(renderer: Raytracer) -> Self {
        let render_channel = channel();
        let renderer_command_channel = channel();

        rendering_thread(renderer, render_channel.0, renderer_command_channel.1);

        RenderingThread {
            render_receiver: render_channel.1,
            renderer_command_sender: renderer_command_channel.0,
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

    pub fn send_command(&self, command: RaytracerCommand) {
        let _ = self.renderer_command_sender.send(command);
    }
}

fn rendering_thread(
    mut renderer: Raytracer,
    render_sender: Sender<Render>,
    command_receiver: Receiver<RaytracerCommand>,
) {
    std::thread::spawn(move || {
        loop {
            while let Ok(command) = command_receiver.try_recv() {
                let _ = renderer.send_command(command);
            }

            let render = renderer.send_command(RaytracerCommand::RenderFrame);

            if let RaytracerResponse::RenderComplete(render) = render {
                let _ = render_sender.send(render);
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
