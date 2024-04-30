use bevy::prelude::*;

use leafwing_input_manager::prelude::*;

use crate::game::ship_plugin::{ActionEventData, Ship};

#[derive(Debug)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, (on_ship_created_add_input, process_inputs));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    ForwardThrust,
    ReverseThrust,
    Aileron,  // Roll
    Elevator, // Pitch
    Rudder,   // Yaw
    Action1,
    Action2,
}

const DEADZONE: f32 = 0.1;

fn default_input_map() -> InputMap<Action> {
    let input_map = InputMap::default()
        // KeyboardMouse
        .insert(Action::ForwardThrust, KeyCode::ShiftLeft)
        .insert(Action::ReverseThrust, KeyCode::ControlLeft)
        .insert(Action::Aileron, VirtualAxis::ad())
        .insert(Action::Elevator, VirtualAxis::ws())
        .insert(
            Action::Rudder,
            VirtualAxis::from_keys(KeyCode::KeyQ, KeyCode::KeyE),
        )
        .insert(Action::Action1, MouseButton::Right)
        .insert(Action::Action2, MouseButton::Left)
        // Gamepad
        .insert(Action::ForwardThrust, GamepadButtonType::RightTrigger2)
        .insert(Action::ReverseThrust, GamepadButtonType::LeftTrigger2)
        .insert(
            Action::Aileron,
            SingleAxis::symmetric(GamepadAxisType::LeftStickX, DEADZONE),
        )
        .insert(
            Action::Elevator,
            SingleAxis::symmetric(GamepadAxisType::LeftStickY, DEADZONE),
        )
        .insert(
            Action::Rudder,
            SingleAxis::symmetric(GamepadAxisType::RightStickX, DEADZONE),
        )
        .insert(Action::Action1, GamepadButtonType::RightTrigger)
        .insert(Action::Action2, GamepadButtonType::LeftTrigger)
        .build();

    input_map
}

#[derive(Component)]
pub struct Controlled;

#[allow(clippy::needless_pass_by_value)]
fn on_ship_created_add_input(mut commands: Commands, query: Query<Entity, Added<Ship>>) {
    for entity in query.iter() {
        _ = commands.entity(entity).insert((
            InputManagerBundle::with_map(default_input_map()),
            Controlled,
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn process_inputs(
    mut commands: Commands,
    query: Query<(&ActionState<Action>, Entity), With<Controlled>>,
) {
    for (action_state, entity) in &query {
        let mut thrust = 0.0;
        let mut roll = 0.0;
        let mut pitch = 0.0;
        let mut yaw = 0.0;
        let mut action1 = 0.0;
        let mut action2 = 0.0;

        if action_state.pressed(&Action::ForwardThrust) {
            // println!("ForwardThrust");
            thrust += action_state.clamped_value(&Action::ForwardThrust);
        }
        if action_state.pressed(&Action::ReverseThrust) {
            // println!("ReverseThrust");
            thrust -= action_state.clamped_value(&Action::ReverseThrust);
        }
        if action_state.pressed(&Action::Aileron) {
            // println!("Aileron");
            roll += action_state.clamped_value(&Action::Aileron);
        }
        if action_state.pressed(&Action::Elevator) {
            // println!("Elevator");
            pitch += action_state.clamped_value(&Action::Elevator);
        }
        if action_state.pressed(&Action::Rudder) {
            // println!("Rudder");
            yaw += action_state.clamped_value(&Action::Rudder);
        }
        if action_state.pressed(&Action::Action1) {
            // println!("Action1");
            action1 += action_state.clamped_value(&Action::Action1);
        }
        if action_state.pressed(&Action::Action2) {
            // println!("Action2");
            action2 += action_state.clamped_value(&Action::Action2);
        }

        _ = commands.entity(entity).insert(ActionEventData {
            thrust,
            roll,
            pitch,
            yaw,
            action1,
            action2,
        });
    }
}
