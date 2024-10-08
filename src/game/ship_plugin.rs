use core::f32::consts::FRAC_PI_2;
use derive_more::AddAssign;
use derive_more::Mul;

use avian3d::prelude::*;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
                FixedUpdate,
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
pub struct Ship {
    auto_balance: bool,
    color: Color,
}

// Player
#[derive(Bundle)]
pub struct ShipBundle {
    ship: Ship,
    spatial: SpatialBundle,
    rigid_body: RigidBody,
    collider: Collider,
    mass_properties: MassPropertiesBundle,
}

impl ShipBundle {
    fn new(mesh: &Mesh, color: Color) -> Self {
        let ship = Ship {
            auto_balance: true,
            color,
        };
        let spatial = SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0));
        let collider = Collider::convex_decomposition_from_mesh(mesh)
            .expect("Failed to create collider from ship_001 mesh");
        let mass_properties =
            MassPropertiesBundle::new_computed(&collider, SHIP_MASS_DENSITY_SCALE);
        Self {
            ship,
            spatial,
            rigid_body: RigidBody::Dynamic,
            collider,
            mass_properties,
            // CollisionLayers::new([Layer::Bots], [Layer::Ground, Layer::Constructed]), // Bots collides with ground, and constructed layers
            // Friction::new(0.0),
            // Restitution::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
            // LinearDamping(0.2),
            // AngularDamping(0.2),
        }
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy System syntax")]
fn setup(mut commands: Commands, ship_assets: Res<ShipAssets>, assets_mesh: Res<Assets<Mesh>>) {
    // Spaceship setup
    if let Some(ship_001) = assets_mesh.get(&ship_assets.ship_001_main) {
        // let collider = Collider::capsule(4.0, 1.0);
        // let collider = Collider::round_cuboid(10.5, 10.5, 5.5, 0.5);
        // Generate pseudo random color from client id.
        let h = 0.0; // (((mesh.to_bits().wrapping_mul(30)) % 360) as f32) / 360.0;
        let s = 0.8;
        let l = 0.5;
        let color = Color::hsl(h, s, l);

        let mesh = ship_001
            .clone()
            .transformed_by(Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2)));
        _ = commands.spawn(ShipBundle::new(&mesh, color));
    }
}

#[derive(Component, Copy, Clone, Default, Debug, Mul, AddAssign)]
// Define an event to represent the spawning of a bot
pub struct ActionEventData {
    pub thrust: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub action1: f32,
    pub action2: f32,
    pub auto_balance: f32,
}

const SHIP_MASS_DENSITY_SCALE: f32 = 0.25;

const PROPULSION_THRUSTERS_STRENGTH: f32 = 10_000.0;
const ANGULAR_THRUSTERS_STRENGTH: f32 = 10_000.0;

// #[expect(clippy::needless_pass_by_value, reason = "Bevy System syntax")]
fn process_actions(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Transform,
            &AngularVelocity,
            &ActionEventData,
            &mut Ship,
        ),
        With<Ship>,
    >,
) {
    for (entity, transform, angular_velocity, action_event_data, mut ship) in &mut query {
        if action_event_data.auto_balance.abs() > 0.5 {
            ship.auto_balance = !ship.auto_balance;
            println!("toggle auto_balance {0}", ship.auto_balance);
        }

        let propulsion_thrusters = ExternalImpulse::new(
            transform.back() * action_event_data.thrust * PROPULSION_THRUSTERS_STRENGTH,
        );

        let roll = auto_balance(
            ship.auto_balance,
            action_event_data.roll,
            angular_velocity,
            transform.back(),
        );
        let pitch = auto_balance(
            ship.auto_balance,
            action_event_data.pitch,
            angular_velocity,
            transform.right(),
        );
        let yaw = auto_balance(
            ship.auto_balance,
            action_event_data.yaw,
            angular_velocity,
            transform.down(),
        );

        let mut angular_trusters = ExternalAngularImpulse::default();
        _ = angular_trusters
            .apply_impulse(transform.back() * roll * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.right() * pitch * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.down() * yaw * ANGULAR_THRUSTERS_STRENGTH);

        // if action_event_data.thrust != 0.0 {
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

fn auto_balance(
    auto_balance_enabled: bool,
    input_value: f32,
    current_angular_velocity: &Vec3,
    axis_vector: Dir3,
) -> f32 {
    // let damping_factor = 0.5;

    if auto_balance_enabled && input_value.abs() < 1e-3 {
        // No significant user input on this axis
        let angular_velocity_along_axis = current_angular_velocity.dot(*axis_vector);
        // -angular_velocity_along_axis * damping_factor * strength
        -angular_velocity_along_axis
    } else {
        input_value
    }
}
