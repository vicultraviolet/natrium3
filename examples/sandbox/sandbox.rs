use natrium3::core::app::App;
use natrium3::core::layer::Layer;
use natrium3::core::window_context::WindowContext;

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
    let mut app = App::new();

    app.window_context = Some(WindowContext::new(String::from("Sandbox")));

    app.push_layer(GameLayer::new());

    app.run();
}

