use natrium3::core::app::App;
use natrium3::core::layer::Layer;

struct GameLayer
{

}

impl GameLayer
{
    pub fn new() -> Self
    {
        Self{

        }
    }
}

impl Layer for GameLayer
{
    
}

fn main()
{
    let mut app = App::new(String::from("Sandbox"));

    app.push_layer(GameLayer::new());

    app.run();
}

