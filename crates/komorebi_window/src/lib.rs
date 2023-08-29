pub mod prelude {}

use komorebi_app::prelude::*;

pub struct WindowPlugin {
    pub close_when_requested: bool,
}

impl Default for WindowPlugin {
    fn default() -> Self {
        WindowPlugin {
            close_when_requested: true,
        }
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
