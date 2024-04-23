use bevy::app::{App, Plugin};

use bytemuck::{Pod, Zeroable};

use bevy_ggrs::prelude::*;

pub type BoxConfig = GgrsConfig<BoxInput>;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct BoxInput(u8);

#[derive(Debug)]
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_plugins(GgrsPlugin::<BoxConfig>::default());

        enable_debug(app);
    }
}

fn enable_debug(_app: &mut App) {
    if cfg!(debug_assertions) {
        // _ = _app.add_plugins(PhysicsDebugPlugin::default());
    }
}
