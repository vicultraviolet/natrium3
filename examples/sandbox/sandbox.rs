use natrium3::core::app::App;
use natrium3::core::layer::Layer;
use natrium3::core::window_context::WindowContext;

struct GameLayer
{
    frame_count: u64
}

impl GameLayer
{
    pub fn new() -> Self
    {
        Self{
            frame_count: 0
        }
    }
}

impl Layer for GameLayer
{
    fn on_tick(&mut self)
    {
        println!("FPS: {}", self.frame_count);     
        self.frame_count = 0;
    }

    fn on_update(&mut self, _dt: f64)
    {
        self.frame_count += 1;
    }
}

fn main()
{
    let mut app = App::new();

    app.set_window_context(WindowContext::new(1000, String::from("Sandbox")));

    app.push_layer(GameLayer::new());

    app.run();
}

