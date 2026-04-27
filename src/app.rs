
pub struct App {
    name: String
}

impl App {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

    pub fn run(&mut self) {
        println!("Hello, {}!", self.name);
    }
}
