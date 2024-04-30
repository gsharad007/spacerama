use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

use bevy_debug_grid::DebugGridPlugin;

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
            .add_plugins(DebugGridPlugin::with_floor_grid())
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, setup);
    }
}

// fn main() {
//     App::new()
//         .insert_resource(DirectionalLightShadowMap { size: 4096 })
//         .add_plugins(DefaultPlugins)
//         .add_systems(Update, animate_light_direction)
//         .run();
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     EnvironmentMapLight {
    //         diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
    //         specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    //         intensity: 250.0,
    //     },
    // ));

    _ = commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            // maximum_distance: 16.0,
            ..default()
        }
        .into(),
        ..default()
    });
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
    //     ..default()
    // });
}
