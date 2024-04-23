use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    diagnostic::FrameTimeDiagnosticsPlugin,
};

use super::ship_plugin::ShipPlugin;

#[allow(clippy::module_name_repetitions)]
pub struct VisualPluginGroup;

impl PluginGroup for VisualPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ShipPlugin)
            .add(FrameTimeDiagnosticsPlugin)
    }
}
