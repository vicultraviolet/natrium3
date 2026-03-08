use std::path::{Path, PathBuf};

use natrium3::core::app::App;
use natrium3::core::asset::{self, Asset};
use natrium3::core::asset_context::Context as AssetContext;
use natrium3::core::layer::Layer;
use natrium3::core::text_asset::TextAsset;
use natrium3::core::window_context::Context as WindowContext;

struct GameLayer
{
    frame_count: u64,
    text_asset: asset::Handle<TextAsset>
}

impl GameLayer
{
    pub fn new(asset_context: &mut AssetContext) -> Self
    {
        let text_asset = asset_context.add(
            String::from("helloWorld"),
            TextAsset::new()
        );

        if let Err(why) = asset_context.save_registry()
        {
            println!("Failed to save asset registry: {}", why);
        }

        if let Some(text_asset) = asset_context.get_mut(&text_asset)
        {
            text_asset.set_data(String::from("Hello world!"));
            if let Err(why) = text_asset.save(Path::new("assets/hello_world.txt"))
            {
                println!("Failed to save text assset: {}", why);
            }
        }

        Self{
            frame_count: 0,
            text_asset
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

    let window_context = WindowContext::new(1000, "Sandbox");
    let mut asset_context = AssetContext::new(PathBuf::from(r"assets/asset_registry.json"));

    app.push_layer(GameLayer::new(&mut asset_context));

    app.set_window_context(window_context);
    app.set_asset_context(asset_context);

    app.run();
}

