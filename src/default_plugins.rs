use komorebi_app::{PluginGroup, PluginGroupBuilder};

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            // .add(komorebi_window::WindowPlugin::default())
            .add(komorebi_winit::WinitPlugin::default());
        group
    }
}
