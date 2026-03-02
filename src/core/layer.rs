use winit::event::WindowEvent;

pub trait Layer
{
   fn on_event(&mut self, _e: &WindowEvent) {}
   fn on_update(&mut self, _dt: f64) {}
   fn on_tick(&mut self) {}
   fn on_draw(&mut self) {}
}

