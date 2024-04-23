use bevy::prelude::*;
use bevy_xpbd_3d::{components::RigidBody, plugins::collision::Collider};

#[derive(Debug)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Controlled;

fn setup(mut commands: Commands) {
    // Spaceship setup
    let collider = Collider::cylinder(1.5, 0.5);
    _ = commands.spawn((
        Ship,
        Controlled,
        RigidBody::Dynamic,
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        collider,
    ));
}
