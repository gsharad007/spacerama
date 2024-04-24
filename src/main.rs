mod game;
mod visual;

use bevy::prelude::*;
use bevy::{
    app::App,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use game::plugin_group::GamePluginGroup;
use visual::plugin_group::VisualPluginGroup;

// const NUM_PLAYERS: usize = 2;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // resolution: (640.0, 480.0).into(),
                        title: "Spacerama".to_owned(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::DX12),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePluginGroup)
        .add_plugins(VisualPluginGroup)
        .run();
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // // Camera setup for first-person view
//     // commands.spawn_bundle(Camera3dBundle {
//     //     transform: Transform::from_xyz(0.0, 1.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
//     //     ..default()
//     // });

//     // Spaceship setup
//     let collider = Collider::cylinder(1.5, 0.5);
//     _ = commands
//         .spawn((
//             RigidBody::Dynamic,
//             SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
//             collider,
//         ))
//         .with_children(|parent| {
//             _ = parent.spawn(PbrBundle {
//                 mesh: meshes.add(Capsule3d::new(0.5, 1.5)),
//                 material: materials.add(Color::WHITE),
//                 ..default()
//             });
//             _ = parent.spawn(Camera3dBundle {
//                 transform: Transform::from_xyz(0.0, 1.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
//                 ..default()
//             });
//         });
// }

// #[allow(clippy::needless_pass_by_value)]
// fn ship_controls(
//     time: Res<Time>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Controlled>>,
// ) {
//     for mut transform in &mut query {
//         let mut direction = Vec3::ZERO;
//         let mut rotation = 0.0;

//         if keyboard_input.pressed(KeyCode::KeyW) {
//             direction.z -= 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyS) {
//             direction.z += 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyA) {
//             rotation += SHIP_ROTATION_SPEED;
//         }
//         if keyboard_input.pressed(KeyCode::KeyD) {
//             rotation -= SHIP_ROTATION_SPEED;
//         }

//         // Update position and orientation
//         transform.rotate_y(rotation);
//         let forward = transform.forward();
//         transform.translation += forward * direction.z * SHIP_SPEED * time.delta_seconds();
//     }
// }
