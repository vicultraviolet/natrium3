use winit::dpi::LogicalSize;
use winit::window::{Window, WindowId};
use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;

use crate::core::app::App;

pub struct WindowContext
{
    window: Option<Window>,
    title: String,
}

impl WindowContext
{
    pub fn new(title: impl Into<String>) -> Self
    {
        Self{
            window: None,
            title: title.into()
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop)
    {
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(1280, 720))
            .with_title(self.title.clone());

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
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
        if let Some(window_context) = &mut self.app.window_context
        {
            window_context.create_window(event_loop);

            if let Some(window) = &window_context.window
            {
                window.request_redraw();
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop)
    {
        if let Some(window_context) = &mut self.app.window_context
        {
            window_context.destroy_window();
        }
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
                self.app.on_update();
                self.app.on_draw();

                if let Some(window_context) = &mut self.app.window_context
                {
                    if let Some(window) = &window_context.window
                    {
                        window.request_redraw();
                    }
                }
            },
            _ => self.app.on_event(&e) 
        }
    }
}

pub fn run_app(app: App)
{
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut handler = Handler{ app };
    let _ = event_loop.run_app(&mut handler);
}

