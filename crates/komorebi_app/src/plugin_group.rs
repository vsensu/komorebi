use crate::{App, AppError, Plugin};
use komorebi_utils::{tracing::debug, tracing::warn};
use std::{any::TypeId, collections::HashMap};

/// Combines multiple [`Plugin`]s into a single group
pub trait PluginGroup: Sized {
    /// Configures the [`Plugin`]s that are to be added
    fn build(self) -> PluginGroupBuilder;

    /// Configures a name for the group which is primarily used for debugging
    fn name() -> String {
        std::any::type_name::<Self>().to_string()
    }

    /// Sets the value of the given [`Plugin`], if it exists
    fn set<T: Plugin>(self, plugin: T) -> PluginGroupBuilder {
        self.build().set(plugin)
    }
}

struct PluginEntry {
    plugin: Box<dyn Plugin>,
    enabled: bool,
}

pub struct PluginGroupBuilder {
    group_name: String,
    plugins: HashMap<TypeId, PluginEntry>,
    order: Vec<TypeId>,
}

impl PluginGroupBuilder {
    pub fn start<PG: PluginGroup>() -> Self {
        Self {
            group_name: PG::name(),
            plugins: Default::default(),
            order: Default::default(),
        }
    }

    /// Adds the plugin [`Plugin`] at the end of this group builder.
    /// If the plugin already exists, it is removed from its previous place.
    #[allow(clippy::should_implement_trait)]
    pub fn add<T: Plugin>(mut self, plugin: T) -> Self {
        let target_index = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert_plugin_state(plugin, target_index);
        self
    }

    pub fn set<T: Plugin>(mut self, plugin: T) -> Self {
        let entry = self.plugins.get_mut(&TypeId::of::<T>()).unwrap_or_else(|| {
            panic!(
                "Plugin {} does not exist in group {}",
                std::any::type_name::<T>(),
                self.group_name
            )
        });
        entry.plugin = Box::new(plugin);
        self
    }

    /// Consumes the [`PluginGroupBuilder`] and [builds](Plugin::build) the contained [`Plugin`]s
    /// in the order specified.
    ///
    /// # Panics
    ///
    /// Panics if one of the plugin in the group was already added to the application.
    pub fn finish(mut self, app: &mut App) {
        for ty in &self.order {
            if let Some(entry) = self.plugins.remove(ty) {
                if entry.enabled {
                    debug!("added plugin: {}", entry.plugin.name());
                    if let Err(AppError::DuplicatePlugin { plugin_name }) =
                        app.add_boxed_plugin(entry.plugin)
                    {
                        panic!(
                            "Error adding plugin {} in group {}: plugin was already added",
                            plugin_name, self.group_name
                        );
                    }
                }
            }
        }
    }

    /// Insert the new plugin as enabled, and removes its previous ordering if it was
    /// already present
    fn upsert_plugin_state<T: Plugin>(&mut self, plugin: T, added_at_index: usize) {
        if let Some(old_entry) = self.plugins.insert(
            TypeId::of::<T>(),
            PluginEntry {
                plugin: Box::new(plugin),
                enabled: true,
            },
        ) {
            if old_entry.enabled {
                warn!(
                    "You are replacing plugin '{}' that was not disabled",
                    old_entry.plugin.name()
                );
            }

            if let Some(to_remove) = self
                .order
                .iter()
                .enumerate()
                .find(|(i, ty)| *i != added_at_index && **ty == TypeId::of::<T>())
                .map(|(i, _)| i)
            {
                self.order.remove(to_remove);
            }
        }
    }
}

impl PluginGroup for PluginGroupBuilder {
    fn build(self) -> PluginGroupBuilder {
        self
    }
}
