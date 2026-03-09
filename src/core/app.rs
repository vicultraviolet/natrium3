use std::time::Duration;
use spin_sleep::sleep;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

use crate::asset::context::Context as AssetContext;
use crate::chrono::accumulator::Accumulator;
use crate::chrono::timekeeper::Timekeeper;
use crate::core::context_info::ContextInfo;
use crate::core::layer::Layer;
use crate::core::window_context::Context as WindowContext;

pub struct App
{
    timekeeper: Timekeeper,
    tick_accumulator: Accumulator,

    window_context: Option<WindowContext>,
    asset_context: Option<AssetContext>,

    layers: Vec<Box<dyn Layer>>,
}

impl App
{
    pub fn new(target_fps: u64) -> Self
    {
        Self{
            timekeeper: Timekeeper::new(target_fps),
            tick_accumulator: Accumulator::new(1.0),
            window_context: None,
            asset_context: None,
            layers: Vec::new()
        }
    }

    pub fn create_context(&mut self, info: ContextInfo)
    {
        match info 
        {
            ContextInfo::Window(title) => self.window_context = Some(WindowContext::new(title)),
            ContextInfo::Asset(registry_path) => self.asset_context = Some(AssetContext::new(registry_path))
        }
    }

    pub fn run(mut self)
    {
        if self.window_context.is_some()
        {
            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);

            event_loop.run_app(&mut self).unwrap();
        }
        else
        {
            for layer in &mut self.layers
            {
                layer.on_tick()
            }

            for layer in &mut self.layers
            {
                layer.on_update(0.0);
            }
        }
    }

    pub fn push_layer<T: Layer + 'static>(&mut self, layer: T)
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

    fn window_loop_instance(&mut self)
    {
        let dt = self.timekeeper.dt();

        let tick_count = self.tick_accumulator.update(dt);
        for _ in 0..tick_count
        {
            for layer in &mut self.layers
            {
                layer.on_tick();
            }
        }
        
        for layer in &mut self.layers
        {
            layer.on_update(dt);
        }

        let window_context = self.window_context.as_ref().unwrap();

        window_context.request_redraw();

        if window_context.is_minimized().unwrap_or(false)
        {
            sleep(Duration::from_millis(20));
        }
        else 
        {
            for layer in &mut self.layers
            {
                layer.on_draw();
            }
        }

        self.timekeeper.pace();
        self.timekeeper.tick();
    }
}

impl ApplicationHandler for App 
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        self.window_context.as_mut().unwrap().create_window(event_loop).request_redraw();
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop)
    {
        self.window_context.as_mut().unwrap().destroy_window();
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
            WindowEvent::RedrawRequested => self.window_loop_instance(),
            _ => self.on_event(&e) 
        }
    }
}

