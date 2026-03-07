use winit::event::WindowEvent;

use crate::core::{asset_context::AssetContext, layer::Layer, window_context::{WindowContext, run_app}};

#[derive(Default)]
pub struct App
{
    window_context: Option<WindowContext>,
    asset_context: Option<AssetContext>,
    layers: Vec<Box<dyn Layer>>,
}

impl App
{
    pub fn new() -> Self
    {
        Self{
            ..Self::default()
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
            self.on_update(0.0);
            self.on_draw();
        }
    }

    pub fn set_window_context(&mut self, c: WindowContext) { self.window_context = Some(c); }
    pub fn destroy_window_context(&mut self) { self.window_context = None; }
    
    pub fn has_window_context(&self) -> bool { self.window_context.is_some() }
    pub fn get_window_context(&mut self) -> &mut WindowContext { self.window_context.as_mut().unwrap() }
    pub fn window_context(&mut self) -> Option<&mut WindowContext> { self.window_context.as_mut() }

    pub fn set_asset_context(&mut self, c: AssetContext) { self.asset_context = Some(c); }
    pub fn destroy_asset_context(&mut self) { self.asset_context = None; }

    pub fn has_asset_context(&self) -> bool { self.asset_context.is_some() }
    pub fn get_asset_context(&mut self) -> &mut AssetContext { self.asset_context.as_mut().unwrap() }
    pub fn asset_context(&mut self) -> Option<&mut AssetContext> { self.asset_context.as_mut() } 

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

    pub fn on_tick(&mut self)
    {
        for layer in &mut self.layers
        {
            layer.on_tick();
        }
    }

    pub fn on_update(&mut self, dt: f64)
    {
        for layer in &mut self.layers
        {
            layer.on_update(dt);
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

