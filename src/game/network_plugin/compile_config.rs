use std::net::{Ipv4Addr, SocketAddr};

use bevy::asset::ron;
use bevy::prelude::*;
use bevy::utils::Duration;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use lightyear::prelude::client::Authentication;
// #[cfg(not(target_family = "wasm"))]
// use lightyear::prelude::client::{SocketConfig, SteamConfig};
use lightyear::prelude::{CompressionConfig, LinkConditionerConfig};

use lightyear::prelude::*;

pub fn read_compile_settings() -> Settings {
    let settings_str = include_str!("../../../assets/config/compile/network_settings.ron");
    read_settings(settings_str)
}

/// We parse the settings.ron file to read the settings
fn read_settings(settings_str: &str) -> Settings {
    ron::de::from_str::<Settings>(settings_str).expect("Could not deserialize the settings file")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub common: CommonSettings,

    /// If true, we will predict the client's entities, but also the ball and other clients' entities!
    /// This is what is done by RocketLeague (see [video](https://www.youtube.com/watch?v=ueEmiDM94IE))
    ///
    /// If false, we will predict the client's entities but simple interpolate everything else.
    pub(crate) predict_all: bool,

    /// By how many ticks an input press will be delayed before we apply client-prediction?
    ///
    /// This can be useful as a tradeoff between input delay and prediction accuracy.
    /// If the input delay is greater than the RTT, then there won't ever be any mispredictions/rollbacks.
    /// See [this article](https://www.snapnet.dev/docs/core-concepts/input-delay-vs-rollback/) for more information.
    pub(crate) input_delay_ticks: u16,

    /// What is the maximum number of ticks that we will rollback for?
    /// After applying input delay, we will try cover `max_prediction_ticks` ticks of latency using client-side prediction
    /// Any more latency beyond that will use more input delay.
    pub(crate) max_prediction_ticks: u16,

    /// If visual correction is enabled, we don't instantly snapback to the corrected position
    /// when we need to rollback. Instead we interpolated between the current position and the
    /// corrected position.
    /// This controls the duration of the interpolation; the higher it is, the longer the interpolation
    /// will take
    pub(crate) correction_ticks_factor: f32,

    /// If true, we will also show the Confirmed entities (on top of the Predicted entities)
    pub(crate) show_confirmed: bool,

    /// Sets server replication send interval in both client and server configs
    pub(crate) server_replication_send_interval: u64,
}

#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct CommonSettings {
    pub server: ServerSettings,
    pub client: ClientSettings,
    pub shared: SharedSettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    /// If true, disable any rendering-related plugins
    pub(crate) headless: bool,

    /// Possibly add a conditioner to simulate network conditions
    pub(crate) conditioner: Option<Conditioner>,

    /// Which transport to use
    pub transport: Vec<ServerTransports>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientSettings {
    /// The client id
    pub(crate) client_id: u64,

    /// The client port to listen on
    pub(crate) client_port: u16,

    /// The ip address of the server
    pub server_addr: Ipv4Addr,

    /// The port of the server
    pub server_port: u16,

    /// Possibly add a conditioner to simulate network conditions
    pub(crate) conditioner: Option<Conditioner>,

    /// Which transport to use
    pub(crate) transport: ClientTransports,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],

    /// compression options
    pub(crate) compression: CompressionConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conditioner {
    /// One way latency in milliseconds
    pub(crate) latency_ms: u16,
    /// One way jitter in milliseconds
    pub(crate) jitter_ms: u16,
    /// Percentage of packet loss
    pub(crate) packet_loss: f32,
}

impl Conditioner {
    pub fn build(&self) -> LinkConditionerConfig {
        LinkConditionerConfig {
            incoming_latency: Duration::from_millis(self.latency_ms as u64),
            incoming_jitter: Duration::from_millis(self.jitter_ms as u64),
            incoming_loss: self.packet_loss,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ClientTransports {
    #[cfg(not(target_family = "wasm"))]
    Udp,
    WebTransport {
        certificate_digest: String,
    },
    WebSocket,
    #[cfg(not(target_family = "wasm"))]
    Steam {
        app_id: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ServerTransports {
    Udp {
        local_port: u16,
    },
    WebTransport {
        local_port: u16,
    },
    WebSocket {
        local_port: u16,
    },
    #[cfg(not(target_family = "wasm"))]
    Steam {
        app_id: u32,
        server_ip: Ipv4Addr,
        game_port: u16,
        query_port: u16,
    },
}
