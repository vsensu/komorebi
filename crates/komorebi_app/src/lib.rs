mod app;
mod plugin;
mod plugin_group;

pub use app::*;
pub use plugin::*;
pub use plugin_group::*;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{app::App, Plugin, PluginGroup};
}
