use bevy::app::{App, Plugin};

#[derive(Debug)]
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        enable_debug(app);
    }
}

fn enable_debug(_app: &mut App) {
    #[cfg(debug_assertions)]
    {
        // _ = _app.add_plugins(PhysicsDebugPlugin::default());
    }
}
