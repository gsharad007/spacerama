use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use bevy_debug_grid::*;

#[derive(Debug)]
pub struct RenderingSetupPlugin;

impl Plugin for RenderingSetupPlugin {
    fn build(&self, app: &mut App) {
        enable_debug(app);
    }
}

fn enable_debug(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        _ = app
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(DebugGridPlugin::with_floor_grid());
    }
}
