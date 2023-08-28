//! An empty application with default plugins.

use komorebi::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
