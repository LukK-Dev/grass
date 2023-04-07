use std::time;

use crate::{
    input_manager::{InputManager, KeyCode},
    renderer::Renderer,
    timer::Timer,
};
use winit::{event_loop, window};

pub struct App {
    event_loop: Option<event_loop::EventLoop<()>>,
    window: window::Window,
    renderer: Renderer,
    input_manager: InputManager,
    should_exit: bool,
    start_instant: time::Instant,
    last_frame_instant: time::Instant,
    timer: Timer,
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
        let start_instant = time::Instant::now();
        let last_frame_instant = time::Instant::now();
        let mut timer = Timer::new(std::time::Duration::from_secs(3), true);
        timer.start();
        Ok(Self {
            event_loop,
            window,
            renderer,
            input_manager,
            should_exit,
            start_instant,
            last_frame_instant,
            timer,
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

        if self.input_manager.is_key_just_pressed(KeyCode::Space) {
            if self.timer.started() {
                self.timer.stop()
            } else {
                self.timer.start()
            }
        }

        self.timer.update(self.time_delta());
        println!("Finished: {}", self.timer.finished());
        println!("{:?}", self.timer.remaining());

        self.input_manager.clear();
        self.last_frame_instant = time::Instant::now()
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        self.renderer.render()?;
        Ok(())
    }

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
