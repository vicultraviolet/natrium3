use winit::window::Window;
use winit::event_loop::ActiveEventLoop;

pub struct Context
{
    window: Option<Window>,
    title: String,
}

impl Context
{
    pub fn new(title: String) -> Self
    {
        Self{
            window: None,
            title
        }
    }

    pub fn has_window(&self) -> bool { self.window.is_some() }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> &mut Window
    {
        let window_attributes = Window::default_attributes()
            .with_title(self.title.clone());

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
        self.window.as_mut().unwrap()
    }

    pub fn destroy_window(&mut self)
    {
        self.window = None;
    }

    pub fn request_redraw(&self) -> bool 
    {
        self.window.as_ref().map(|window| window.request_redraw()).is_some()
    }

    pub fn get_size(&self) -> Option<(u32, u32)>
    {
        self.window.as_ref().map(|window| (window.inner_size().width, window.inner_size().height ))
    }

    pub fn title(&self) -> &str { &self.title }
    pub fn set_title(&mut self, title: String) -> Option<String>
    {
        match self.window.as_mut()
        {
            Some(window) =>
            {
                self.title = title;
                window.set_title(&self.title);
                None
            }
            None => Some(title) 
        }
    }

    pub fn has_focus(&self) -> Option<bool> 
    {
        self.window.as_ref().map(|window| window.has_focus())
    }

    pub fn is_minimized(&self) -> Option<bool>
    {
        match self.window.as_ref()
        {
            Some(window) =>
            {
                window.is_minimized()
            }
            None => None
        }
    }
}

