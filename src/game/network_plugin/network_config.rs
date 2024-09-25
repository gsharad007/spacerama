use std::net::{Ipv4Addr, SocketAddr};

use bevy::asset::ron;
use bevy::prelude::{default, Resource};
use bevy::utils::Duration;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(not(target_family = "wasm"))]
use async_compat::Compat;
#[cfg(not(target_family = "wasm"))]
use bevy::tasks::IoTaskPool;

use lightyear::prelude::client::Authentication;
#[cfg(not(target_family = "wasm"))]
use lightyear::prelude::client::*;
use lightyear::prelude::{CompressionConfig, LinkConditionerConfig};

use lightyear::prelude::{client, server};

use super::compile_config::ServerTransports;
use super::compile_config::{ClientTransports, CommonSettings, Conditioner, SharedSettings};

#[allow(dead_code)]
pub(crate) fn build_server_netcode_config(
    conditioner: Option<&Conditioner>,
    shared: &SharedSettings,
    transport_config: server::ServerTransport,
) -> server::NetConfig {
    let conditioner = conditioner.map_or(None, |c| {
        Some(LinkConditionerConfig {
            incoming_latency: Duration::from_millis(c.latency_ms as u64),
            incoming_jitter: Duration::from_millis(c.jitter_ms as u64),
            incoming_loss: c.packet_loss,
        })
    });
    let netcode_config = server::NetcodeConfig::default()
        .with_protocol_id(shared.protocol_id)
        .with_key(shared.private_key);
    let io_config = server::IoConfig {
        transport: transport_config,
        conditioner,
        compression: shared.compression,
    };
    server::NetConfig::Netcode {
        config: netcode_config,
        io: io_config,
    }
}

/// Parse the settings into a list of `NetConfig` that are used to configure how the lightyear server
/// listens for incoming client connections
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_server_net_configs(settings: &CommonSettings) -> Vec<server::NetConfig> {
    settings
        .server
        .transport
        .iter()
        .map(|t| match t {
            ServerTransports::Udp { local_port } => build_server_netcode_config(
                settings.server.conditioner.as_ref(),
                &settings.shared,
                server::ServerTransport::UdpSocket(SocketAddr::new(
                    Ipv4Addr::UNSPECIFIED.into(),
                    *local_port,
                )),
            ),
            #[cfg(feature = "webtransport")]
            ServerTransports::WebTransport { local_port } => {
                // this is async because we need to load the certificate from io
                // we need async_compat because wtransport expects a tokio reactor
                let certificate = IoTaskPool::get()
                    .scope(|s| {
                        s.spawn(Compat::new(async {
                            server::Identity::load_pemfiles(
                                "../certificates/cert.pem",
                                "../certificates/key.pem",
                            )
                            .await
                            .unwrap()
                        }));
                    })
                    .pop()
                    .unwrap();
                let digest = certificate.certificate_chain().as_slice()[0].hash();
                println!("Generated self-signed certificate with digest: {}", digest);
                build_server_netcode_config(
                    settings.server.conditioner.as_ref(),
                    &settings.shared,
                    server::ServerTransport::WebTransportServer {
                        server_addr: SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *local_port),
                        certificate,
                    },
                )
            }
            #[cfg(feature = "websocket")]
            ServerTransports::WebSocket { local_port } => build_server_netcode_config(
                settings.server.conditioner.as_ref(),
                &settings.shared,
                server::ServerTransport::WebSocketServer {
                    server_addr: SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *local_port),
                },
            ),
            #[cfg(all(feature = "steam", not(target_family = "wasm")))]
            ServerTransports::Steam {
                app_id,
                server_ip,
                game_port,
                query_port,
            } => server::NetConfig::Steam {
                steamworks_client: None,
                config: server::SteamConfig {
                    app_id: *app_id,
                    socket_config: server::SocketConfig::Ip {
                        server_ip: *server_ip,
                        game_port: *game_port,
                        query_port: *query_port,
                    },
                    max_clients: 16,
                    ..default()
                },
                conditioner: settings
                    .server
                    .conditioner
                    .as_ref()
                    .map_or(None, |c| Some(c.build())),
            },
        })
        .collect()
}

/// Build a netcode config for the client
pub(crate) fn build_client_netcode_config(
    client_id: u64,
    server_addr: SocketAddr,
    conditioner: Option<&Conditioner>,
    shared: &SharedSettings,
    transport_config: client::ClientTransport,
) -> client::NetConfig {
    let conditioner = conditioner.map_or(None, |c| Some(c.build()));
    let auth = Authentication::Manual {
        server_addr,
        client_id,
        private_key: shared.private_key,
        protocol_id: shared.protocol_id,
    };
    let netcode_config = client::NetcodeConfig::default();
    let io_config = client::IoConfig {
        transport: transport_config,
        conditioner,
        compression: shared.compression,
    };
    client::NetConfig::Netcode {
        auth,
        config: netcode_config,
        io: io_config,
    }
}

/// Parse the settings into a `NetConfig` that is used to configure how the lightyear client
/// connects to the server
pub fn get_client_net_config(settings: &CommonSettings, client_id: u64) -> client::NetConfig {
    let server_addr = SocketAddr::new(
        settings.client.server_addr.into(),
        settings.client.server_port,
    );
    let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.client.client_port);
    match &settings.client.transport {
        #[cfg(not(target_family = "wasm"))]
        ClientTransports::Udp => build_client_netcode_config(
            client_id,
            server_addr,
            settings.client.conditioner.as_ref(),
            &settings.shared,
            client::ClientTransport::UdpSocket(client_addr),
        ),
        #[cfg(feature = "webtransport")]
        ClientTransports::WebTransport { certificate_digest } => build_client_netcode_config(
            client_id,
            server_addr,
            settings.client.conditioner.as_ref(),
            &settings.shared,
            client::ClientTransport::WebTransportClient {
                client_addr,
                server_addr,
                #[cfg(target_family = "wasm")]
                certificate_digest: certificate_digest.to_string().replace(":", ""),
            },
        ),
        #[cfg(feature = "websocket")]
        ClientTransports::WebSocket => build_client_netcode_config(
            client_id,
            server_addr,
            settings.client.conditioner.as_ref(),
            &settings.shared,
            client::ClientTransport::WebSocketClient { server_addr },
        ),
        #[cfg(all(feature = "steam", not(target_family = "wasm")))]
        ClientTransports::Steam { app_id } => client::NetConfig::Steam {
            steamworks_client: None,
            config: SteamConfig {
                socket_config: SocketConfig::Ip { server_addr },
                app_id: *app_id,
            },
            conditioner: settings
                .server
                .conditioner
                .as_ref()
                .map_or(None, |c| Some(c.build())),
        },
    }
}
