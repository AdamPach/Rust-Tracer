use crate::core::render::Render;
use crate::rendering::{Raytracer, RaytracerCommand, RaytracerResponse};
use std::ops::Deref;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::sync::{Arc, Mutex};

pub enum RenderingThreadCommand {
    SendCommand(RaytracerCommand),
    StartRendering,
    StopRendering,
}

pub enum RenderingThreadResponse {
    CommandResponse(RaytracerResponse),
    RenderingStarted,
    RenderingStopped,
}

pub struct RenderingThread {
    response_receiver: Receiver<anyhow::Result<RenderingThreadResponse>>,
    command_sender: Sender<RenderingThreadCommand>,
    last_render: Arc<Mutex<Option<(Render, bool)>>>,
}

impl RenderingThread {
    pub fn new(renderer: Raytracer) -> Self {
        let render_channel = channel();
        let renderer_command_channel = channel();

        let last_render = Arc::new(Mutex::new(None));

        rendering_thread(
            renderer,
            render_channel.0,
            renderer_command_channel.1,
            last_render.clone(),
        );

        RenderingThread {
            response_receiver: render_channel.1,
            command_sender: renderer_command_channel.0,
            last_render,
        }
    }

    pub fn try_read_responses(
        &mut self,
    ) -> Result<anyhow::Result<RenderingThreadResponse>, TryRecvError> {
        match self.response_receiver.try_recv() {
            Ok(response) => Ok(response),
            Err(TryRecvError::Empty) => Err(TryRecvError::Empty),
            Err(TryRecvError::Disconnected) => panic!("render thread disconnected"),
        }
    }

    pub fn get_last_render(&self) -> Option<Render> {
        let mut last_render = self.last_render.lock().unwrap();

        let mut result: Option<Render> = None;

        match last_render.deref() {
            Some((render, true)) => {
                result = Some(render.clone());
                *last_render = Some((render.clone(), false));
            }
            _ => {}
        }

        result
    }

    pub fn send_command(&self, command: RenderingThreadCommand) {
        let _ = self.command_sender.send(command);
    }
}

fn rendering_thread(
    mut renderer: Raytracer,
    response_sndr: Sender<anyhow::Result<RenderingThreadResponse>>,
    command_rcv: Receiver<RenderingThreadCommand>,
    last_render: Arc<Mutex<Option<(Render, bool)>>>,
) {
    std::thread::spawn(move || {
        loop {
            let running = matches!(renderer, Raytracer::Running(_));

            let command = if running {
                match command_rcv.try_recv() {
                    Ok(command) => Some(command),
                    Err(TryRecvError::Empty) => None,
                    Err(TryRecvError::Disconnected) => panic!("render thread disconnected"),
                }
            } else {
                let Ok(command) = command_rcv.recv() else {
                    break;
                };

                Some(command)
            };

            if let Some(command) = command {
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
                let _ = response_sndr.send(response);
                continue;
            }

            if running {
                let Raytracer::Running(running_renderer) = &mut renderer else {
                    panic!("Renderer should be running at this point");
                };

                let render = running_renderer.render_image();

                let mut last_render = last_render.lock().unwrap();
                *last_render = Some((render, true));
            }
        }
    });
}
