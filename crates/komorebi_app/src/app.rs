use std::{
    collections::HashSet,
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
};

// use crate::window;
use crate::{Plugin, Plugins};

use komorebi_utils::tracing::debug;

pub(crate) enum AppError {
    DuplicatePlugin { plugin_name: String },
}

pub struct App {
    // pub window: window::Window,
    plugin_registry: Vec<Box<dyn Plugin>>,
    plugin_name_added: HashSet<String>,
    /// prevent incorrect calls to `App::run()` from `Plugin::build()`
    building_plugin_depth: usize,
}

// Dummy plugin used to temporary hold the place in the plugin registry
struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, _app: &mut App) {}
}

impl App {
    pub fn new() -> App {
        App {
            // window: window::Window::new(),
            plugin_registry: Default::default(),
            plugin_name_added: Default::default(),
            building_plugin_depth: 0,
        }
    }

    pub fn run(&self) {
        println!("hello komorebi");
        // self.window.run();
    }

    pub(crate) fn add_boxed_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
    ) -> Result<&mut Self, AppError> {
        debug!("added plugin: {}", plugin.name());
        if plugin.is_unique() && !self.plugin_name_added.insert(plugin.name().to_string()) {
            Err(AppError::DuplicatePlugin {
                plugin_name: plugin.name().to_string(),
            })?;
        }

        let plugin_pos_in_registry = self.plugin_registry.len();
        self.plugin_registry.push(Box::new(PlaceholderPlugin));

        self.building_plugin_depth += 1;
        let result = catch_unwind(AssertUnwindSafe(|| plugin.build(self)));
        self.building_plugin_depth -= 1;
        if let Err(payload) = result {
            resume_unwind(payload);
        }
        self.plugin_registry[plugin_pos_in_registry] = plugin;
        Ok(self)
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        plugins.add_to_app(self);
        self
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
