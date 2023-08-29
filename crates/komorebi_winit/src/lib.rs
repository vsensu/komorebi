mod window;
mod winit_windows;

use komorebi_app::prelude::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget},
};

#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        let event_handler = move |event: Event<()>,
                                  event_loop: &EventLoopWindowTarget<()>,
                                  control_flow: &mut ControlFlow| {
            match event {
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        };

        let event_loop = EventLoopBuilder::<()>::with_user_event().build();
        let window = winit_windows::WinitWindows::create_window(&event_loop);
        event_loop.run(event_handler);
    }
}
