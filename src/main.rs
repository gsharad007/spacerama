mod apps;
mod cli;
mod game;
mod visual;

use std::time::Duration;

use autodefault::autodefault;
use bevy::prelude::*;

use clap::Parser;

use apps::Apps;
use cli::CommandLineArguments;

#[autodefault]
fn main() -> AppExit {
    let args = CommandLineArguments::parse();
    println!("Command Line Arguments: {args}");

    let compiled_network_settings = game::network_plugin::compiled_config::read_compiled_settings();
    println!("Compile Settings: {compiled_network_settings:?}");

    let mut apps = Apps::new(&compiled_network_settings.common, &args)
        .with_server_replication_send_interval(Duration::from_millis(
            compiled_network_settings.server_replication_send_interval,
        ));

    _ = apps.add_lightyear_plugins();

    // apps.add_user_plugins(
    //     VisualPlugins,
    //         ,
    //     GamePlugins,
    // );

    // run the app
    apps.run()

    // App::new()
    //     .add_plugins(
    //         DefaultPlugins
    //             .set(WindowPlugin {
    //                 primary_window: Some(Window {
    //                     // resolution: (640.0, 480.0).into(),
    //                     title: "Spacerama".to_owned(),
    //                 }),
    //             })
    //             .set(RenderPlugin {
    //                 render_creation: RenderCreation::Automatic(WgpuSettings {
    //                     backends: Some(Backends::DX12),
    //                 }),
    //             }),
    //     )
    //     .add_plugins(GamePlugins)
    //     .add_plugins(VisualPlugins)
    //     .insert_resource(args)
    //     .run()
}
