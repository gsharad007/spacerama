use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

#[derive(Debug)]
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .init_state::<MainState>()
            .init_state::<InGameState>()
            .configure_sets(
                Update,
                (
                    FrameSystemsSet::World.before(FrameSystemsSet::Input),
                    FrameSystemsSet::Input.before(FrameSystemsSet::Player),
                    FrameSystemsSet::Player.before(FrameSystemsSet::Physics),
                    FrameSystemsSet::Physics,
                )
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            .configure_sets(
                FixedUpdate,
                (
                    FrameSystemsSet::World.before(FrameSystemsSet::Input),
                    FrameSystemsSet::Input.before(FrameSystemsSet::Player),
                    FrameSystemsSet::Player.before(FrameSystemsSet::Physics),
                    FrameSystemsSet::Physics,
                )
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            .add_loading_state(
                LoadingState::new(MainState::Loading).continue_to_state(MainState::InGame),
            );
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainState {
    // TODO: Add Startup and MainMenu in Phase 2 or later
    // Startup,
    // MainMenu,
    #[default]
    Loading,
    InGame,
}

// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum GameModeState {
//     #[default]
//     NotInGame,
//     Singleplayer,
//     Multiplayer,
// }

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameState {
    Paused,
    #[default]
    Running,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FrameSystemsSet {
    World,
    Input,
    Player,
    Physics,
}
