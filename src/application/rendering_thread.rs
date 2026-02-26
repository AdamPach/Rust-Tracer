use crate::core::render::Render;
use crate::rendering::{Raytracer, RaytracerCommand, RaytracerResponse};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};

pub enum RenderingThreadCommand {
    SendCommand(RaytracerCommand),
    StartRendering,
    StopRendering,
}

pub enum RenderingThreadResponse {
    CommandResponse(RaytracerResponse),
    Render(Render),
    RenderingStarted,
    RenderingStopped,
}

pub struct RenderingThread {
    render_receiver: Receiver<anyhow::Result<RenderingThreadResponse>>,
    renderer_command_sender: Sender<RenderingThreadCommand>,
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

    pub fn try_recv_response(
        &mut self,
    ) -> Result<anyhow::Result<RenderingThreadResponse>, TryRecvError> {
        match self.render_receiver.try_recv() {
            Ok(render) => {
                let mut response = render;

                while let Ok(new_render) = self.render_receiver.try_recv() {
                    response = new_render;
                }

                Ok(response)
            }
            Err(TryRecvError::Empty) => Err(TryRecvError::Empty),
            Err(TryRecvError::Disconnected) => panic!("render thread disconnected"),
        }
    }

    pub fn send_command(&self, command: RenderingThreadCommand) {
        let _ = self.renderer_command_sender.send(command);
    }
}

fn rendering_thread(
    mut renderer: Raytracer,
    render_sender: Sender<anyhow::Result<RenderingThreadResponse>>,
    command_receiver: Receiver<RenderingThreadCommand>,
) {
    std::thread::spawn(move || {
        loop {
            while let Ok(command) = command_receiver.try_recv() {
                let response = match command {
                    RenderingThreadCommand::SendCommand(command) => {
                        let response = renderer.send_command(command);

                        response.map(RenderingThreadResponse::CommandResponse)
                    }
                    RenderingThreadCommand::StartRendering => {
                        renderer = renderer.run();

                        Ok(RenderingThreadResponse::RenderingStarted)
                    }
                    RenderingThreadCommand::StopRendering => {
                        renderer = renderer.stop();

                        Ok(RenderingThreadResponse::RenderingStopped)
                    }
                };

                let _ = render_sender.send(response);
            }

            let response = match &mut renderer {
                Raytracer::Running(renderer) => {
                    Ok(RenderingThreadResponse::Render(renderer.render_image()))
                }
                Raytracer::NotRunning(_) => continue,
            };

            let _ = render_sender.send(response);
        }
    });
}
