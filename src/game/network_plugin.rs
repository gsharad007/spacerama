use bevy::prelude::*;

use bevy_ggrs::{prelude::*, GgrsConfig, Session};

use bevy_matchbox::{matchbox_socket::SingleChannel, prelude::*, MatchboxSocket};

use bevy_ggrs::{prelude::*, GgrsConfig, Session};

use crate::cli::CommandLineArguments;

use super::{
    action_event_data::ActionEventData,
    states_plugin::{InGameState, MainState},
};

// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
type Config = GgrsConfig<u8, PeerId>;

use crate::cli::CommandLineArguments;

use super::{
    action_event_data::ActionEventData,
    states_plugin::{InGameState, MainState},
};

// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
pub type ActionEventDataConfig = GgrsConfig<ActionEventData, PeerId>;

#[derive(Debug)]
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_plugins(GgrsPlugin::<ActionEventDataConfig>::default())
            .add_systems(PreStartup, parse_commandlinearguements)
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(
                Update,
                wait_for_players
                    .run_if(in_state(MainState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );

        enable_debug(app);
    }
}

fn enable_debug(_app: &mut App) {
    #[cfg(debug_assertions)]
    {
        // _ = _app.add_plugins(PhysicsDebugPlugin::default());
    }
}

#[derive(Resource, Debug)]
pub struct NetworkingRuntimeConfig {
    pub session_id: String,
    pub player_count: u8,
    // pub synctest: bool,
}

#[allow(clippy::needless_pass_by_value)]
fn parse_commandlinearguements(mut commands: Commands, args: Res<CommandLineArguments>) {
    commands.insert_resource(NetworkingRuntimeConfig {
        session_id: args.session_id.clone(),
        player_count: args.player_count,
        // synctest: args.synctest,
    });
}

#[allow(clippy::needless_pass_by_value)]
fn start_matchbox_socket(mut commands: Commands, runtime_config: Res<NetworkingRuntimeConfig>) {
    let session_id = runtime_config.session_id.clone();
    let player_count = runtime_config.player_count;
    let room_url = format!("ws://127.0.0.1:3536/{session_id}?next={player_count}");
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

#[allow(clippy::needless_pass_by_value)]
fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    runtime_config: Res<NetworkingRuntimeConfig>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    _ = socket.update_peers();
    let players = socket.players();

    let num_players = runtime_config.player_count as usize;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = SessionBuilder::<ActionEventDataConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).expect("failed to take channel");

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    let game_session = Session::P2P(ggrs_session);
    commands.insert_resource(game_session);

    info!("Game Session Started!");
}
