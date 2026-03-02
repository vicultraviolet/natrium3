use winit::dpi::LogicalSize;
use winit::window::{Window, WindowId};
use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::application::ApplicationHandler;

use crate::core::layer::Layer;

pub struct App
{
    window: Option<Window>,
    title: String,
    layers: Vec<Box<dyn Layer>>,
}

impl App
{
    pub fn new(title: String) -> Self
    {
        Self{
            window: None,
            title,
            layers: Vec::new()
        }
    }

    pub fn run(self)
    {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut handler = Handler{ app: self };
        let _ = event_loop.run_app(&mut handler);
    }

    pub fn push_layer(&mut self, layer: impl Layer + 'static)
    {
        self.layers.push(Box::new(layer));
    }

    fn on_event(&mut self, e: &WindowEvent)
    {
        for layer in &mut self.layers
        {
            layer.on_event(e);
        }
    }

    fn on_update(&mut self)
    {
        for layer in &mut self.layers
        {
            layer.on_update(0.0);
        }
    }

    fn on_draw(&mut self)
    {
        for layer in &mut self.layers
        {
            layer.on_draw();
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
    app: App
}

impl ApplicationHandler for Handler
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        self.app.create_window(event_loop);

        if let Some(window) = &self.app.window
        {
            window.request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop)
    {
       self.app.destroy_window(); 
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

                if let Some(window) = &self.app.window
                {
                    window.request_redraw();
                }
            },
            _ => self.app.on_event(&e) 
        }
    }
}

