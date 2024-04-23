use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    diagnostic::LogDiagnosticsPlugin,
};

use super::{
    network_plugin::NetworkingPlugin, physics_plugin::PhysicsPlugin, ship_plugin::ShipPlugin,
};

#[allow(clippy::module_name_repetitions)]
pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPlugin)
            .add(NetworkingPlugin)
            .add(ShipPlugin)
            .add(LogDiagnosticsPlugin::default())
    }
}
