use autodefault::autodefault;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::{
    ship_plugin::Ship,
    states_plugin::{InGameState, MainState},
};

#[derive(Debug)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .configure_loading_state(
                LoadingStateConfig::new(MainState::Loading).load_collection::<ShipAssets>(),
            )
            .add_systems(
                FixedUpdate,
                on_ship_created_add_visuals
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );
    }
}

#[derive(AssetCollection, Resource)]
struct ShipAssets {
    #[asset(path = "models/ships/ship_001.glb#Scene0")]
    ship_001_scene: Handle<Scene>,
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy System syntax")]
#[autodefault]
fn on_ship_created_add_visuals(
    mut commands: Commands,
    ship_assets: Res<ShipAssets>,
    query: Query<Entity, Added<Ship>>,
) {
    for entity in query.iter() {
        _ = commands.entity(entity).with_children(|parent| {
            // let mesh = Capsule3d::new(0.5, 1.5);
            _ = parent.spawn(SceneBundle {
                scene: ship_assets.ship_001_scene.clone(),
            });
            _ = parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 4.5, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
            });
        });
    }
}
