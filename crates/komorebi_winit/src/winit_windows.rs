use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

pub struct WinitWindows {}

impl WinitWindows {
    pub fn create_window(
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> winit::window::Window {
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        window
    }
}
