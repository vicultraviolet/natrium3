use std::any::TypeId;
use std::path::PathBuf;

use natrium3::core::app::App;
use natrium3::core::context_info::ContextInfo;
use natrium3::core::layer::Layer;
use natrium3::ecs::archetype::Archetype;
use natrium3::ecs::component::Component;
use natrium3::ecs::entity::Entity;

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

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u16);

fn main()
{
    {
        let mut archetype = Archetype::new(vec![
            TypeId::of::<Name>(), TypeId::of::<Age>() 
        ]);

        archetype.push2(
            Entity{ index: 0, generation: 0},
            (
                Name(String::from("Eugene")),
                Age(57)
            )
        );

        if let Some(name) = archetype.get_component::<Name>(0) &&
            let Some(age) = archetype.get_component::<Age>(0)
        {
            println!("{}: age {}", name.0, age.0);
        }
    }

    let mut app = App::new(1000);

    app.create_context(ContextInfo::Window(String::from("Sandbox")));
    app.create_context(ContextInfo::Asset(PathBuf::from("assets/asset_registry.json")));

    app.push_layer(GameLayer::new());

    app.run();
}

