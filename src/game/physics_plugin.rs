use avian3d::prelude::*;

use bevy::app::FixedUpdate;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;

#[derive(Debug)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_plugins(PhysicsPlugins::new(FixedUpdate))
            .insert_resource(Gravity(Vec3::ZERO)); // Disable Gravity since we are doing outer space experience for now

        enable_debug(app);
    }
}

fn enable_debug(app: &mut App) {
    #[cfg(feature = "debug_physics")]
    {
        _ = app.add_plugins(PhysicsDebugPlugin::default());
    }
}
