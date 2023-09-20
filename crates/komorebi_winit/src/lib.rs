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
        let event_loop = EventLoopBuilder::<()>::with_user_event().build();
        app.insert_non_send_resource(event_loop);
    }
}
