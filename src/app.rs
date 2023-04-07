use std::time;

use crate::input_manager::{InputManager, KeyCode};
use winit::{event_loop, window};

pub struct App {
    event_loop: Option<event_loop::EventLoop<()>>,
    window: window::Window,
    input_manager: InputManager,
    should_exit: bool,
    start_instant: time::Instant,
    last_frame_instant: time::Instant,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let event_loop = event_loop::EventLoop::new();
        let window = window::WindowBuilder::new().build(&event_loop)?;
        let event_loop = Some(event_loop);

        let input_manager = InputManager::new();
        let should_exit = false;
        let start_instant = time::Instant::now();
        let last_frame_instant = time::Instant::now();
        Ok(Self {
            event_loop,
            window,
            input_manager,
            should_exit,
            start_instant,
            last_frame_instant,
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

            self.last_frame_instant = time::Instant::now()
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
                self.input_manager.update(event)
            }
            winit::event::Event::MainEventsCleared => {
                self.update();
                self.window.request_redraw()
            }
            winit::event::Event::RedrawRequested(_) => self.draw(),
            _ => (),
        }
    }

    fn update(&mut self) {
        if self.input_manager.is_key_pressed(KeyCode::Escape) {
            self.should_exit = true
        }

        let mut x = 0;
        for i in 1..100000 {
            x += x % i
        }

        println!("{:?} FPS", self.fps())
    }

    fn draw(&mut self) {}

    fn time_since_start(&self) -> time::Duration {
        time::Instant::now() - self.start_instant
    }

    fn time_delta(&self) -> time::Duration {
        time::Instant::now() - self.last_frame_instant
    }

    fn fps(&self) -> u32 {
        (1.0 / self.time_delta().as_secs_f32()) as u32
    }
}
