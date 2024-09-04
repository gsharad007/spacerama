use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;

use naia_bevy_server::transport::Socket;
use naia_bevy_server::{Plugin as NaiaServerPlugin, ReceiveEvents, Server, ServerConfig};
use naia_bevy_server::{RoomKey, UserKey};

use systems::{events, init};

use crate::networking_shared::protocol::protocol;

#[derive(Debug)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let mut server_config = ServerConfig::default();
        server_config.connection.disconnection_timeout_duration = Duration::from_secs(10);

        _ = app
            // Plugins
            .add_plugins(TaskPoolPlugin::default())
            .add_plugins(TypeRegistrationPlugin::default())
            .add_plugins(FrameCountPlugin::default())
            // this is needed to avoid running the server at uncapped FPS
            .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)))
            .add_plugins(LogPlugin::default())
            .add_plugins(NaiaServerPlugin::new(server_config, protocol()))
            // Startup System
            .add_systems(Startup, init)
            // Receive Server Events
            .add_systems(
                Update,
                (
                    events::auth_events,
                    events::connect_events,
                    events::disconnect_events,
                    events::error_events,
                    events::tick_events,
                    events::spawn_entity_events,
                    events::despawn_entity_events,
                    events::publish_entity_events,
                    events::unpublish_entity_events,
                    events::insert_component_events,
                    events::update_component_events,
                    events::remove_component_events,
                )
                    .chain()
                    .in_set(ReceiveEvents),
            );
    }
}

#[derive(Resource)]
pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_square_map: HashMap<UserKey, Entity>,
    pub square_to_user_map: HashMap<Entity, UserKey>,
}

#[derive(Resource)]
struct Lobby {
    room_key: RoomKey,
}

fn init(mut commands: Commands, mut server: Server) {
    info!("Server is stating up...");
    // Naia Server initialization
    let server_addresses = ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );
    let socket = Socket::new(&server_addresses, server.socket_config());
    server.listen(socket);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Init Global Resource
    let global = Global {
        main_room_key,
        user_to_square_map: HashMap::new(),
        square_to_user_map: HashMap::new(),
    };

    // Insert Global Resource
    commands.insert_resource(global);
}

fn setup(mut commands: Commands) {
    // Create socket
    let socket = Socket::bind("127.0.0.1:14191").unwrap();

    // Server configuration
    let server_addrs = ServerAddrs::new(
        "127.0.0.1:14191".parse().unwrap(),
        "127.0.0.1:14192".parse().unwrap(),
        "127.0.0.1:14193".parse().unwrap(),
    );

    let mut server = Server::new(ServerConfig::default(), server_addrs, socket);

    // Create a room (lobby)
    let lobby = server.create_room().unwrap();
    commands.insert_resource(Lobby { room_key: lobby });
    commands.insert_resource(server);
}

fn update(mut server: ResMut<Server>, lobby: Res<Lobby>, time: Res<Time>) {
    server.update(&time);

    // Handle room management and player connections
    for client_key in server.client_keys() {
        server
            .room_add_client(&lobby.room_key, &client_key)
            .unwrap();
    }
}
