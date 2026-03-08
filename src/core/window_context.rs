use winit::dpi::LogicalSize;
use winit::window::{Window, WindowId};
use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;

use crate::chrono::accumulator::Accumulator;
use crate::chrono::timekeeper::Timekeeper;
use crate::core::app::App;

pub struct Context
{
    timekeeper: Timekeeper,
    tick_accumulator: Accumulator,
    window: Option<Window>,
    title: String,
}

impl Context
{
    pub fn new(target_fps: u64, title: impl Into<String>) -> Self
    {
        Self{
            timekeeper: Timekeeper::new(target_fps),
            tick_accumulator: Accumulator::new(1.0),
            window: None,
            title: title.into()
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> &mut Window
    {
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(1280, 720))
            .with_title(self.title.clone());

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
        self.window.as_mut().unwrap()
    }

    fn destroy_window(&mut self)
    {
        self.window = None;
    }
}

struct Handler
{
    app: App,
}

impl ApplicationHandler for Handler
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        self.app.get_window_context().create_window(event_loop).request_redraw();
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop)
    {
        self.app.get_window_context().destroy_window();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        e: WindowEvent,
    ) {
        match e
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested =>
            {
                let dt = self.app.get_window_context().timekeeper.dt();
                let tick_count = self.app.get_window_context().tick_accumulator.update(dt);

                for _ in 0..tick_count
                {
                    self.app.on_tick();
                }

                self.app.on_update(dt);

                self.app.on_draw();

                if let Some(window) = &self.app.get_window_context().window
                {
                    window.request_redraw();
                }

                self.app.get_window_context().timekeeper.pace();
                self.app.get_window_context().timekeeper.tick();
            },
            _ => self.app.on_event(&e) 
        }
    }
}

pub fn run_app(mut app: App)
{
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    app.get_window_context().timekeeper.tick();

    let mut handler = Handler{ app };
    let _ = event_loop.run_app(&mut handler);
}

