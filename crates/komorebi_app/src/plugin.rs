use crate::App;
use downcast_rs::{impl_downcast, Downcast};

pub trait Plugin: Downcast {
    /// Configure the `App`
    fn build(&self, app: &mut App);

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {}

    fn cleanup(&self, _app: &mut App) {}

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// Whether this plugin is unique. If `true`, then only one instance of this plugin can be added
    fn is_unique(&self) -> bool {
        true
    }
}

impl_downcast!(Plugin);

/// Types that represent a set of [`Plugin`]s
pub trait Plugins<Marker>: sealed::Plugins<Marker> {}

impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}

mod sealed {
    use crate::{App, AppError, Plugin, PluginGroup};

    pub trait Plugins<Marker> {
        fn add_to_app(self, app: &mut App);
    }

    pub struct PluginMarker;
    pub struct PluginGroupMarker;

    impl<P: Plugin> Plugins<PluginMarker> for P {
        fn add_to_app(self, app: &mut App) {
            if let Err(AppError::DuplicatePlugin { plugin_name }) =
                app.add_boxed_plugin(Box::new(self))
            {
                panic!("Duplicate plugin: {plugin_name}");
            }
        }
    }

    impl<P: PluginGroup> Plugins<PluginGroupMarker> for P {
        fn add_to_app(self, app: &mut App) {
            self.build().finish(app);
        }
    }
}
