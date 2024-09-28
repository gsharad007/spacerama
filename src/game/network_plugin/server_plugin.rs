use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy::utils::HashMap;
use client::Rollback;
use leafwing_input_manager::action_diff::ActionDiff;
use leafwing_input_manager::prelude::*;
use lightyear::client::connection;
use lightyear::prelude::client::{Confirmed, Predicted};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::shared::tick_manager;

// use crate::protocol::*;
// use crate::shared;
// use crate::shared::ApplyInputsQuery;
// use crate::shared::ApplyInputsQueryItem;
// use crate::shared::{apply_action_state_to_player_movement, color_from_id, FixedSet};

// Plugin for server-specific logic
pub struct ServerPlugin;
// {
// pub(crate) predict_all: bool,
//}

#[derive(Resource)]
pub struct Global {
    predict_all: bool,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(Global {
        //     predict_all: self.predict_all,
        // });
        app.add_systems(Startup, (start_server, init));
        // app.add_systems(
        //     PreUpdate,
        //     // this system will replicate the inputs of a client to other clients
        //     // so that a client can predict other clients
        //     replicate_inputs.after(MainSet::EmitEvents),
        // );
        // // the physics/FixedUpdates systems that consume inputs should be run in this set
        // app.add_systems(
        //     FixedUpdate,
        //     (player_movement, shared::shared_player_firing)
        //         .chain()
        //         .in_set(FixedSet::Main),
        // );
        // app.add_systems(
        //     Update,
        //     (
        //         handle_connections,
        //         update_player_metrics.run_if(on_timer(Duration::from_secs(1))),
        //     ),
        // );

        // app.add_systems(
        //     FixedUpdate,
        //     handle_hit_event
        //         .run_if(on_event::<BulletHitEvent>())
        //         .after(shared::process_collisions),
        // );
    }
}

/// System to start the server at Startup
fn start_server(mut commands: Commands) {
    commands.start_server();
}

fn init(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Server",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::End,
            ..default()
        }),
    );
    // // the balls are server-authoritative
    // const NUM_BALLS: usize = 6;
    // for i in 0..NUM_BALLS {
    //     let radius = 10.0 + i as f32 * 4.0;
    //     let angle: f32 = i as f32 * (TAU / NUM_BALLS as f32);
    //     let pos = Vec2::new(125.0 * angle.cos(), 125.0 * angle.sin());
    //     commands.spawn(BallBundle::new(radius, pos, css::GOLD.into()));
    // }
}
