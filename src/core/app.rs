use winit::event::WindowEvent;

use crate::core::{layer::Layer, window_context::{WindowContext, run_app}};

pub struct App
{
    pub window_context: Option<WindowContext>,
    layers: Vec<Box<dyn Layer>>,
}

impl App
{
    pub fn new() -> Self
    {
        Self{
            window_context: None,
            layers: Vec::new(),
        }
    }

    pub fn run(mut self)
    {
        if self.window_context.is_some()
        {
            run_app(self);
        }
        else
        {
            self.on_update();
            self.on_draw();
        }
    }

    pub fn push_layer(&mut self, layer: impl Layer + 'static)
    {
        self.layers.push(Box::new(layer));
    }

    pub fn on_event(&mut self, e: &WindowEvent)
    {
        for layer in &mut self.layers
        {
            layer.on_event(e);
        }
    }

    pub fn on_update(&mut self)
    {
        for layer in &mut self.layers
        {
            layer.on_update(0.0);
        }
    }

    pub fn on_draw(&mut self)
    {
        for layer in &mut self.layers
        {
            layer.on_draw();
        }
    }
}

