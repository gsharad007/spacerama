use bevy::prelude::*;

use crate::game::ship_plugin::Ship;

#[derive(Debug)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Update, on_ship_created_add_visuals);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn on_ship_created_add_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, Added<Ship>>,
) {
    let mesh = Capsule3d::new(0.5, 1.5);
    for entity in query.iter() {
        _ = commands.entity(entity).with_children(|parent| {
            _ = parent.spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::WHITE),
                ..default()
            });
            _ = parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });
    }
}
