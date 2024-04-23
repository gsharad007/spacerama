use bevy::prelude::*;

use crate::game::ship_plugin::Controlled;


#[derive(Debug)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Update, ship_controls);
    }
}

const SHIP_SPEED: f32 = 10.0;
const SHIP_ROTATION_SPEED: f32 = 0.1;

#[allow(clippy::needless_pass_by_value)]
fn ship_controls(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<Transform, With<Controlled>>,
) {

    let translation = keyboard_input.get_pressed().fold(translation, |acc, &key| {
        acc + match key {
            KeyCode::KeyW => Vec2::new(0., 1.),
            KeyCode::KeyS => -Vec2::new(0., 1.),
            KeyCode::KeyA => -Vec2::new(1., 0.),
            KeyCode::KeyD => Vec2::new(1., 0.),
            _ => Vec2::ZERO,
        }
    });


    for transform in &mut query {
        let mut direction = Vec3::ZERO;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation += SHIP_ROTATION_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation -= SHIP_ROTATION_SPEED;
        }

        // Update position and orientation
        transform.rotate_y(rotation);
        let forward = transform.forward();
        transform.translation += forward * direction.z * SHIP_SPEED * time.delta_seconds();
    }
}
