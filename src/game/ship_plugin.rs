use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::{ExternalAngularImpulse, ExternalImpulse, RigidBody},
    plugins::collision::Collider,
};

#[derive(Debug)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_systems(Startup, setup)
            .add_systems(Update, process_actions);
    }
}

#[derive(Component)]
pub struct Ship;

fn setup(mut commands: Commands) {
    // Spaceship setup
    let collider = Collider::cylinder(1.5, 0.5);
    _ = commands.spawn((
        Ship,
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        RigidBody::Dynamic,
        collider,
    ));
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

const PROPULSION_THRUSTERS_STRENGTH: f32 = 10.0;
const ANGULAR_THRUSTERS_STRENGTH: f32 = 0.1;

#[allow(clippy::needless_pass_by_value)]
fn process_actions(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &ActionEventData), With<Ship>>,
) {
    for (entity, transform, action_event_data) in &query {
        let propulsion_thrusters =
            ExternalImpulse::new(transform.back() * action_event_data.thrust * PROPULSION_THRUSTERS_STRENGTH);

        let mut angular_trusters = ExternalAngularImpulse::default();
        _ = angular_trusters
            .apply_impulse(transform.back() * action_event_data.roll * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.right() * action_event_data.pitch * ANGULAR_THRUSTERS_STRENGTH)
            .apply_impulse(transform.down() * action_event_data.yaw * ANGULAR_THRUSTERS_STRENGTH);

        println!("ActionEventData: {action_event_data:?}");
        println!(
            "propulsion_thrusters: {propulsion_thrusters:?}, angular_trusters: {angular_trusters:?}",
        );

        _ = commands
            .entity(entity)
            .insert((propulsion_thrusters, angular_trusters));

        // // Spawn Action1 Action2
        // commands.spawn(bundle)
    }
}
