use core::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::{
    components::{ExternalAngularImpulse, ExternalImpulse, MassPropertiesBundle, RigidBody},
    plugins::collision::Collider,
};

use super::states_plugin::{FrameSystemsSet, InGameState, MainState};

#[derive(Debug)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .configure_loading_state(
                LoadingStateConfig::new(MainState::Loading).load_collection::<ShipAssets>(),
            )
            .add_systems(OnEnter(MainState::InGame), setup)
            .add_systems(
                Update,
                process_actions
                    .in_set(FrameSystemsSet::Player)
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );
    }
}

#[derive(AssetCollection, Resource)]
struct ShipAssets {
    #[asset(path = "models/ships/ship_001.glb#Mesh0/Primitive0")]
    ship_001_main: Handle<Mesh>,
}

#[derive(Component)]
pub struct Ship;

// #[derive(Resource)]
// struct ShipModelsAssets(Handle<LoadedFolder>);

// fn pre_load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(ShipModelsAssets(asset_server.load("models/ships/ship_001.glb")));
// }

#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, ship_assets: Res<ShipAssets>, assets_mesh: Res<Assets<Mesh>>) {
    // Spaceship setup
    if let Some(ship_001) = assets_mesh.get(&ship_assets.ship_001_main) {
        // let collider = Collider::capsule(4.0, 1.0);
        // let collider = Collider::round_cuboid(10.5, 10.5, 5.5, 0.5);
        let mesh = ship_001
            .clone()
            .transformed_by(Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2)));
        let collider = Collider::convex_decomposition_from_mesh(&mesh)
            .expect("Failed to create collider from ship_001 mesh");
        let mass_bundle = MassPropertiesBundle::new_computed(&collider, SHIP_MASS_DENSITY_SCALE);
        _ = commands.spawn((
            Ship,
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            RigidBody::Dynamic,
            collider,
            mass_bundle,
            // CollisionLayers::new([Layer::Bots], [Layer::Ground, Layer::Constructed]), // Bots collides with ground, and constructed layers
            // Friction::new(0.0),
            // Restitution::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
            // LinearDamping(0.2),
            // AngularDamping(0.2),
        ));
    }
}

// #[derive(Debug)]
// pub enum Action {
//     ForwardThrust,
//     ReverseThrust,
//     Aileron,  // Roll
//     Elevator, // Pitch
//     Rudder,   // Yaw
//     Action1,
//     Action2,
// }

#[derive(Component, Debug)]
// Define an event to represent the spawning of a bot
pub struct ActionEventData {
    pub thrust: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub action1: f32,
    pub action2: f32,
}

const SHIP_MASS_DENSITY_SCALE: f32 = 0.25;

const PROPULSION_THRUSTERS_STRENGTH: f32 = 10000.0;
const ANGULAR_THRUSTERS_STRENGTH: f32 = 1000.0;

#[allow(clippy::needless_pass_by_value)]
fn process_actions(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &ActionEventData), With<Ship>>,
) {
    for (entity, transform, action_event_data) in &query {
        let propulsion_thrusters = ExternalImpulse::new(
            transform.back() * action_event_data.thrust * PROPULSION_THRUSTERS_STRENGTH,
        );

        let mut angular_trusters = ExternalAngularImpulse::default();
        _ = angular_trusters
            .apply_impulse(transform.back() * action_event_data.roll * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.right() * action_event_data.pitch * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.down() * action_event_data.yaw * ANGULAR_THRUSTERS_STRENGTH);

        // if action_event_data.roll != 0.0 {
        //     println!("Transform: {transform:?}");
        //     println!("ActionEventData: {action_event_data:?}");
        //     println!(
        //         "propulsion_thrusters: {propulsion_thrusters:?}, angular_trusters: {angular_trusters:?}",
        //     );
        // }

        _ = commands
            .entity(entity)
            .insert((propulsion_thrusters, angular_trusters));

        // // Spawn Action1 Action2
        // commands.spawn(bundle)
    }
}
