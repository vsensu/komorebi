use crate::window;

pub struct App {
    pub window: window::Window,
}

impl App {
    pub fn new() -> App {
        App {
            window: window::Window::new(),
        }
    }

    pub fn run(&self) {
        println!("hello komorebi");
        self.window.run();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
