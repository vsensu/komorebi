// use crate::window;

pub struct App {
    // pub window: window::Window,
}

impl App {
    pub fn new() -> App {
        App {
            // window: window::Window::new(),
        }
    }

    pub fn run(&self) {
        println!("hello komorebi");
        // self.window.run();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
