use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

use bevy_debug_grid::DebugGridPlugin;

use crate::game::states_plugin::MainState;

#[derive(Debug)]
pub struct RenderingSetupPlugin;

impl Plugin for RenderingSetupPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(OnEnter(MainState::InGame), setup);

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

fn setup(mut commands: Commands) {
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
}
