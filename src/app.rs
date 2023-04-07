use std::time;

use crate::{
    input_manager::{InputManager, KeyCode},
    renderer::Renderer,
    timer::Timer,
    timing::Timing,
};
use winit::{event_loop, window};

pub struct App {
    event_loop: Option<event_loop::EventLoop<()>>,
    window: window::Window,
    renderer: Renderer,
    input_manager: InputManager,
    should_exit: bool,
    timing: Timing,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        tracing_subscriber::fmt::init();

        let event_loop = event_loop::EventLoop::new();
        let window = window::WindowBuilder::new().build(&event_loop)?;
        let event_loop = Some(event_loop);

        let renderer = Renderer::new(&window).await?;

        let input_manager = InputManager::new();
        let should_exit = false;
        let timing = Timing::new();
        Ok(Self {
            event_loop,
            window,
            renderer,
            input_manager,
            should_exit,
            timing,
        })
    }

    pub fn run(mut self) {
        let event_loop = std::mem::replace(&mut self.event_loop, None).unwrap();

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            self.handle_event(event);

            if self.should_exit {
                control_flow.set_exit()
            }
        });
    }

    fn handle_event(&mut self, event: winit::event::Event<()>) {
        match event {
            winit::event::Event::WindowEvent { window_id, event }
                if window_id == self.window.id() =>
            {
                if let winit::event::WindowEvent::CloseRequested = event {
                    self.should_exit = true;
                }
                if let winit::event::WindowEvent::Resized(size) = event {
                    self.renderer.resize(size.width, size.height).unwrap()
                }
                self.input_manager.update(event)
            }
            winit::event::Event::MainEventsCleared => {
                self.update();
                self.window.request_redraw()
            }
            winit::event::Event::RedrawRequested(_) => self.draw().unwrap(),
            _ => (),
        }
    }

    fn update(&mut self) {
        if self.input_manager.is_key_pressed(KeyCode::Escape) {
            self.should_exit = true
        }

        self.input_manager.clear();
        self.timing.update();
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        self.renderer.render()?;
        Ok(())
    }
}
