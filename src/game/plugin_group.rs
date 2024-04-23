use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

use super::{network_plugin::NetworkingPlugin, physics_plugin::PhysicsPlugin};

#[allow(clippy::module_name_repetitions)]
pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPlugin)
            .add(NetworkingPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(LogDiagnosticsPlugin::default())
    }
}
