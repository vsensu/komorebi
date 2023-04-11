use crate::ecs;
use crate::window;

pub struct App {
    pub world: ecs::World,
    pub window: window::Window,
}

impl App {
    pub fn new() -> App {
        App {
            world: ecs::World {},
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
