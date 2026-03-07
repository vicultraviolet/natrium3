use std::path::{Path, PathBuf};

use natrium3::core::app::App;
use natrium3::core::asset::Asset;
use natrium3::core::asset_context::AssetContext;
use natrium3::core::layer::Layer;
use natrium3::core::text_asset::TextAsset;
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

    app.set_window_context(WindowContext::new(1000, "Sandbox"));
    app.set_asset_context(AssetContext::new(PathBuf::from(r"assets/asset_registry.json")));

    let hello_world = app.get_asset_context().new_asset("helloWorld", TextAsset::new);

    hello_world.set_data(String::from("Hello, world!"));
    let _ = hello_world.save(Path::new("assets/hello_world.txt"));
    if let Err(e) = app.get_asset_context().save_registry()
    {
        println!("Failed to save asset registry: {}", e)
    }

    app.push_layer(GameLayer::new());

    app.run();
}

