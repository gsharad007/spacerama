use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::utils::Duration;
use leafwing_input_manager::prelude::*;
// use lightyear::inputs::leafwing::input_buffer::InputBuffer;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use lightyear::shared::replication::components::Controlled;
use lightyear::shared::tick_manager;

use crate::game::states_plugin::MainState;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::InGame), init);
        //     app.add_systems(
        //         PreUpdate,
        //         handle_connection
        //             .after(MainSet::Receive)
        //             .before(PredictionSet::SpawnPrediction),
        //     );
        //     // all actions related-system that can be rolled back should be in FixedUpdate schedule
        //     app.add_systems(
        //         FixedUpdate,
        //         (
        //             // in host-server, we don't want to run the movement logic twice
        //             // disable this because we also run the movement logic in the server
        //             player_movement.run_if(not(is_host_server)),
        //             // we don't spawn bullets during rollback.
        //             // if we have the inputs early (so not in rb) then we spawn,
        //             // otherwise we rely on normal server replication to spawn them
        //             shared_player_firing.run_if(not(is_in_rollback)),
        //         )
        //             .chain()
        //             .in_set(FixedSet::Main),
        //     );
        //     app.add_systems(
        //         Update,
        //         (
        //             add_ball_physics,
        //             add_bullet_physics, // TODO better to scheduled right after replicated entities get spawned?
        //             handle_new_player,
        //         ),
        //     );
        //     app.add_systems(
        //         FixedUpdate,
        //         handle_hit_event
        //             .run_if(on_event::<BulletHitEvent>())
        //             .after(process_collisions),
        //     );

        //     #[cfg(target_family = "wasm")]
        //     app.add_systems(
        //         Startup,
        //         |mut settings: ResMut<lightyear::client::web::KeepaliveSettings>| {
        //             // the show must go on, even in the background.
        //             let keepalive = 1000. / FIXED_TIMESTEP_HZ;
        //             info!("Setting webworker keepalive to {keepalive}");
        //             settings.wake_delay = keepalive;
        //         },
        //     );
    }
}

// Startup system for the client
pub(crate) fn init(mut commands: Commands) {
    commands.connect_client();
}
