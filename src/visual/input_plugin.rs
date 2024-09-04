use autodefault::autodefault;
use bevy::{prelude::*, utils::HashMap};

use leafwing_input_manager::{action_state::ActionKindData, buttonlike::ButtonState, prelude::*};

use crate::game::{
    ship_plugin::{ActionEventData, Ship},
    states_plugin::{FrameSystemsSet, InGameState, MainState},
};

#[derive(Debug)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                Update,
                (on_ship_created_add_input, process_inputs)
                    .in_set(FrameSystemsSet::Input)
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );
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
    AutoBalance,
}

const DEADZONE: f32 = 0.1;

fn default_input_map() -> InputMap<Action> {
    let input_map = InputMap::default()
        // KeyboardMouse
        .with(Action::ForwardThrust, KeyCode::ShiftLeft)
        .with(Action::ReverseThrust, KeyCode::ControlLeft)
        .with(Action::Aileron, KeyboardVirtualAxis::AD)
        .with(Action::Elevator, KeyboardVirtualAxis::WS)
        .with(
            Action::Rudder,
            KeyboardVirtualAxis::from_keys(KeyCode::KeyQ, KeyCode::KeyE),
        )
        .with(Action::Action1, MouseButton::Right)
        .with(Action::Action2, MouseButton::Left)
        .with(Action::AutoBalance, KeyCode::KeyB)
        // Gamepad
        .with(Action::ForwardThrust, GamepadButtonType::RightTrigger2)
        .with(Action::ReverseThrust, GamepadButtonType::LeftTrigger2)
        .with_axis(
            Action::Aileron,
            GamepadControlAxis::LEFT_X.with_deadzone_symmetric(DEADZONE),
        )
        .with_axis(
            Action::Elevator,
            GamepadControlAxis::LEFT_Y.with_deadzone_symmetric(DEADZONE),
        )
        .with_axis(
            Action::Rudder,
            GamepadControlAxis::RIGHT_X.with_deadzone_symmetric(DEADZONE),
        )
        .with(Action::Action1, GamepadButtonType::RightTrigger)
        .with(Action::Action2, GamepadButtonType::LeftTrigger);
    // .build();

    input_map
}

// #[derive(Default)]
// struct ActionEventStateData {
//     button_state: ButtonState,
//     action_event_data: ActionEventData,
// }

#[autodefault]
fn default_action_map() -> HashMap<Action, (ButtonState, ActionEventData)> {
    let action_map: HashMap<_, _> = [
        (
            Action::ForwardThrust,
            (ButtonState::Pressed, ActionEventData { thrust: 1.0 }),
        ),
        (
            Action::ReverseThrust,
            (ButtonState::Pressed, ActionEventData { thrust: -1.0 }),
        ),
        (
            Action::Aileron,
            (ButtonState::Pressed, ActionEventData { roll: 1.0 }),
        ),
        (
            Action::Elevator,
            (ButtonState::Pressed, ActionEventData { pitch: 1.0 }),
        ),
        (
            Action::Rudder,
            (ButtonState::Pressed, ActionEventData { yaw: 1.0 }),
        ),
        (
            Action::Action1,
            (ButtonState::Pressed, ActionEventData { action1: 1.0 }),
        ),
        (
            Action::Action2,
            (ButtonState::Pressed, ActionEventData { action2: 1.0 }),
        ),
        (
            Action::AutoBalance,
            (
                ButtonState::JustPressed,
                ActionEventData { auto_balance: 1.0 },
            ),
        ),
    ]
    .iter()
    .copied()
    .collect();

    action_map
}

#[derive(Component)]
pub struct Controlled {
    action_map: HashMap<Action, (ButtonState, ActionEventData)>,
}

#[allow(clippy::needless_pass_by_value)]
fn on_ship_created_add_input(mut commands: Commands, query: Query<Entity, Added<Ship>>) {
    for entity in query.iter() {
        _ = commands.entity(entity).insert((
            InputManagerBundle::with_map(default_input_map()),
            Controlled {
                action_map: default_action_map(),
            },
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn process_inputs(
    mut commands: Commands,
    query: Query<(&ActionState<Action>, &Controlled, Entity), With<Controlled>>,
) {
    for (action_state, controlled, entity) in &query {
        let mut action_data = ActionEventData::default();

        for (action, &(button_state_expected, action_event_data)) in &controlled.action_map {
            if let ActionKindData::Button(action_button_data) = &action_state
                .action_data(action)
                .expect("Action data not found for the given action")
                .kind_data
            {
                let action_button_state = action_button_data.state;
                if action_state.pressed(action) && action_button_state == button_state_expected {
                    let value = action_state.clamped_value(action);
                    action_data += action_event_data * value;
                }
            }
        }

        _ = commands.entity(entity).insert(action_data);
    }
}
