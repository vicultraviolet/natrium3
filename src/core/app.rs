pub struct App
{
    running: bool
}

impl App
{
    pub fn new() -> Self
    {
        App{
            running: false
        }
    }

    pub fn run(&mut self)
    {
        self.running = true;
        println!("Running!");
    }
}