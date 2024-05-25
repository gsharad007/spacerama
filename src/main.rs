mod cli;
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

use clap::Parser;

use cli::CommandLineArguments;
use game::plugin_group::GamePluginGroup;
use visual::plugin_group::VisualPluginGroup;

// const NUM_PLAYERS: usize = 2;

fn main() {
    let args = CommandLineArguments::parse();
    println!("Command Line Arguments: {args}");

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
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePluginGroup)
        .add_plugins(VisualPluginGroup)
        .insert_resource(args)
        .run();
}
