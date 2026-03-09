use std::path::PathBuf;

use natrium3::core::app::App;
use natrium3::core::context_info::ContextInfo;
use natrium3::core::layer::Layer;

struct GameLayer
{
    frame_count: u64,
}

impl GameLayer
{
    pub fn new() -> Self
    {
        Self{
            frame_count: 0,
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
    let mut app = App::new(1000);

    app.create_context(ContextInfo::Window(String::from("Sandbox")));
    app.create_context(ContextInfo::Asset{ registry_path: PathBuf::from("assets/asset_registry.json") });

    app.push_layer(GameLayer::new());

    app.run();
}

