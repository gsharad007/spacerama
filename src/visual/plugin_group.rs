use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::{rendering_setup_plugin::RenderingSetupPlugin, ship_plugin::ShipPlugin};
use crate::visual::input_plugin::InputPlugin;

#[allow(clippy::module_name_repetitions)]
pub struct VisualPluginGroup;

impl PluginGroup for VisualPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RenderingSetupPlugin)
            .add(ShipPlugin)
            .add(InputPlugin)
    }
}
