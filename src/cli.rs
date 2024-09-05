use derive_more::Display;

use bevy::ecs::system::Resource;
use clap::Parser;

#[derive(Parser, Resource, Display, Clone)]
#[display(
    "Session Id: {}, Player Count: {}, Sync Test: {}",
    session_id,
    player_count,
    synctest
)]
#[command(version, about, long_about = None)]
pub struct CommandLineArguments {
    /// the session id for current p2p session
    #[clap(long, default_value = "spacerama")]
    pub session_id: String,
    /// the number of players for current p2p session
    #[clap(long, default_value = "1")]
    pub player_count: u8,

    /// runs the game in synctest mode
    #[clap(long)]
    pub synctest: bool,
}
