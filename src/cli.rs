use bevy::ecs::system::Resource;
use clap::Parser;

#[derive(Parser, Resource, Debug, Clone)]
pub struct CommandLineArguments {
    /// runs the game in synctest mode
    #[clap(long)]
    pub synctest: bool,
}
