use std::net::SocketAddr;
use std::time::Duration;

use autodefault::autodefault;
use bevy::app::PluginGroupBuilder;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::DefaultPlugins;
use bevy::{
    app::App,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{Window, WindowPlugin},
};

use lightyear::prelude::client::PredictionConfig;
use lightyear::prelude::client::ClientConfig;
use lightyear::prelude::*;
use lightyear::prelude::{client, server};
use lightyear::server::config::ServerConfig;
use lightyear::transport::LOCAL_SOCKET;

use crate::cli::CommandLineArguments;
use crate::game::network_plugin::compiled_config::{CommonSettings, Settings};
use crate::game::network_plugin::network_config::{
    build_client_netcode_config, get_client_net_config,
};
use crate::game::network_plugin::network_config::{
    build_server_netcode_config, get_server_net_configs,
};
use crate::game::network_plugin::shared_config::{shared_config, REPLICATION_INTERVAL};
use crate::game::plugin_group::GamePlugins;
use crate::visual::plugin_group::VisualPlugins;

pub struct HeadlessServerPlugins;

impl PluginGroup for HeadlessServerPlugins {
    #[autodefault]
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(MinimalPlugins)
            .add(StatesPlugin)
            .add(LogPlugin {
                level: Level::INFO,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            })
    }
}

pub struct ServerPlugins;

impl PluginGroup for ServerPlugins {
    #[autodefault]
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(HeadlessServerPlugins)
            .add_group(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            // resolution: (640.0, 480.0).into(),
                            title: "Spacerama".to_owned(),
                        }),
                    })
                    .set(RenderPlugin {
                        render_creation: RenderCreation::Automatic(WgpuSettings {
                            backends: Some(Backends::DX12),
                        }),
                    })
                    .set(LogPlugin {
                        level: Level::INFO,
                        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
                    }),
            )
            .add_group(VisualPlugins)
    }
}

pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    #[autodefault]
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            // resolution: (640.0, 480.0).into(),
                            title: "Spacerama".to_owned(),
                        }),
                    })
                    .set(RenderPlugin {
                        render_creation: RenderCreation::Automatic(WgpuSettings {
                            backends: Some(Backends::DX12),
                        }),
                    })
                    .set(LogPlugin {
                        level: Level::INFO,
                        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
                    }),
            )
            .add_group(GamePlugins)
            .add_group(VisualPlugins)
    }
}

/// App that is Send.
/// Used as a convenient workaround to send an App to a separate thread,
/// if we know that the App doesn't contain ``NonSend`` resources.
struct SendApp(App);

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for SendApp {}
impl SendApp {
    fn run(&mut self) {
        self.0.run();
    }
}

/// Apps that will be returned from the `build_apps` function
///
/// The configs are also included so that the user can modify them if needed, before running the app.
pub enum Apps {
    /// A single app that contains both the Client and Server plugins
    HostServer {
        app: App,
        server_config: ServerConfig,
        client_config: ClientConfig,
    },
    /// Two apps (Client and Server) that will run in separate threads
    ServerAndClient {
        server_app: App,
        server_config: ServerConfig,
        client_app: App,
        client_config: ClientConfig,
    },
    /// A single app that contains only the ``ServerPlugins``
    Server { app: App, config: ServerConfig },
    /// A single app that contains only the ``ClientPlugins``
    Client { app: App, config: ClientConfig },
}

impl Apps {
    /// Build the apps with the given settings and CLI options.
    pub fn new(settings: &CommonSettings, cli: &CommandLineArguments) -> Self {
        match cli {
            #[cfg(not(target_family = "wasm"))]
            CommandLineArguments::HostServer { client_id } => {
                let client_net_config = client::NetConfig::Local { id: *client_id };
                Self::combined_app(settings, vec![], client_net_config)
            }
            #[cfg(not(target_family = "wasm"))]
            CommandLineArguments::ServerAndClient { client_id } => {
                // we will communicate between the client and server apps via channels
                let (from_server_send, from_server_recv) = crossbeam_channel::unbounded();
                let (to_server_send, to_server_recv) = crossbeam_channel::unbounded();
                let transport_config = client::ClientTransport::LocalChannel {
                    recv: from_server_recv,
                    send: to_server_send,
                };

                // create client app
                let net_config = build_client_netcode_config(
                    *client_id,
                    // when communicating via channels, we need to use the address `LOCAL_SOCKET` for the server
                    LOCAL_SOCKET,
                    settings.client.conditioner.as_ref(),
                    &settings.shared,
                    transport_config,
                );
                let client = Self::client_app(settings, net_config);

                // create server app
                let extra_transport_configs = vec![server::ServerTransport::Channels {
                    // even if we communicate via channels, we need to provide a socket address for the client
                    channels: vec![(LOCAL_SOCKET, to_server_recv, from_server_send)],
                }];
                let server = Self::server_app(settings, extra_transport_configs);

                match (client, server) {
                    (
                        Self::Client {
                            app: client_app,
                            config: client_config,
                        },
                        Self::Server {
                            app: server_app,
                            config: server_config,
                        },
                    ) => Self::ServerAndClient {
                        client_app,
                        client_config,
                        server_app,
                        server_config,
                    },
                    _ => panic!("Expected Apps::Client and Apps::Server"),
                }
            }
            #[cfg(not(target_family = "wasm"))]
            CommandLineArguments::Server => Self::server_app(settings, vec![]),
            CommandLineArguments::Client { client_id } => {
                // use the cli-provided client id if it exists, otherwise use the settings client id
                let net_config = get_client_net_config(settings, *client_id);
                Self::client_app(settings, net_config)
            }
        }
    }

    /// Set the `server_replication_send_interval` on client and server apps.
    /// Use to overwrite the default [`SharedConfig`] value in the settings file.
    pub fn with_server_replication_send_interval(mut self, replication_interval: Duration) -> Self {
        _ = self
            .update_lightyear_client_config(|cc: &mut ClientConfig| {
                cc.shared.server_replication_send_interval = replication_interval;
            })
            .update_lightyear_server_config(|sc: &mut ServerConfig| {
                // the server replication currently needs to be overwritten in both places...
                sc.shared.server_replication_send_interval = replication_interval;
                sc.replication.send_interval = replication_interval;
            });
        self
    }

    /// Add the lightyear [`ClientPlugins`] and [`ServerPlugins`] plugin groups to the app.
    ///
    /// This can be called after any modifications to the [`ClientConfig`] and [`ServerConfig`]
    /// have been applied.
    pub fn add_lightyear_plugins(&mut self) -> &mut Self {
        match self {
            Self::Client { app, config } => {
                _ = app.add_plugins(client::ClientPlugins {
                    config: config.clone(),
                });
            }
            Self::Server { app, config } => {
                _ = app.add_plugins(server::ServerPlugins {
                    config: config.clone(),
                });
            }
            Self::ServerAndClient {
                client_app,
                server_app,
                client_config,
                server_config,
            } => {
                _ = client_app.add_plugins(client::ClientPlugins {
                    config: client_config.clone(),
                });
                _ = server_app.add_plugins(server::ServerPlugins {
                    config: server_config.clone(),
                });
            }
            Self::HostServer {
                app,
                client_config,
                server_config,
            } => {
                // TODO: currently we need ServerPlugins to run first, because it adds the
                //  SharedPlugins. not ideal
                _ = app.add_plugins(client::ClientPlugins {
                    config: client_config.clone(),
                });
                _ = app.add_plugins(server::ServerPlugins {
                    config: server_config.clone(),
                });
            }
        }
        self
    }

    /// Add the client, server, and shared user-provided plugins to the app
    pub fn add_user_plugins(
        &mut self,
        client_plugin: impl Plugin,
        server_plugin: impl Plugin,
        shared_plugin: impl Plugin + Clone,
    ) -> &mut Self {
        match self {
            Self::Client { app, .. } => {
                _ = app.add_plugins((client_plugin, shared_plugin));
            }
            Self::Server { app, .. } => {
                _ = app.add_plugins((server_plugin, shared_plugin));
            }
            Self::ServerAndClient {
                client_app,
                server_app,
                ..
            } => {
                _ = client_app.add_plugins((client_plugin, shared_plugin.clone()));
                _ = server_app.add_plugins((server_plugin, shared_plugin));
            }
            Self::HostServer { app, .. } => {
                _ = app.add_plugins((client_plugin, server_plugin, shared_plugin));
            }
        }
        self
    }

    /// Apply a function to update the [`ClientConfig`]
    pub fn update_lightyear_client_config(
        &mut self,
        f: impl FnOnce(&mut ClientConfig),
    ) -> &mut Self {
        match self {
            Self::Client { config, .. } => {
                f(config);
            }
            Self::Server { .. } => {}
            Self::ServerAndClient { client_config, .. } => {
                f(client_config);
            }
            Self::HostServer { client_config, .. } => {
                f(client_config);
            }
        }
        self
    }

    /// Apply a function to update the [`ServerConfig`]
    pub fn update_lightyear_server_config(
        &mut self,
        f: impl FnOnce(&mut ServerConfig),
    ) -> &mut Self {
        match self {
            Self::Client { .. } => {}
            Self::Server { config, .. } => {
                f(config);
            }
            Self::ServerAndClient { server_config, .. } => {
                f(server_config);
            }
            Self::HostServer { server_config, .. } => {
                f(server_config);
            }
        }
        self
    }

    /// Start running the apps.
    pub fn run(self) -> AppExit {
        match self {
            Self::Client { mut app, .. } => app.run(),
            Self::Server { mut app, .. } => app.run(),
            Self::ServerAndClient {
                mut client_app,
                server_app,
                ..
            } => {
                let mut send_app = SendApp(server_app);
                std::thread::spawn(move || send_app.run());
                client_app.run()
            }
            Self::HostServer { mut app, .. } => app.run(),
        }
    }

    /// Build the client app with the `ClientPlugins` added.
    /// Takes in a `net_config` parameter so that we configure the network transport.
    // #[autodefault]
    fn client_app(_settings: &CommonSettings, net_config: client::NetConfig) -> Self {
        let mut app = App::new();
        _ = app.add_plugins(ClientPlugins);

        let config = ClientConfig {
            shared: shared_config(Mode::Separate),
            net: net_config,
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            ..default()
        };
        Self::Client { app, config }
    }

    /// Build the server app with the `ServerPlugins` added.
    #[cfg(not(target_family = "wasm"))]
    fn server_app(
        settings: &CommonSettings,
        extra_transport_configs: Vec<server::ServerTransport>,
    ) -> Self {
        let mut app = App::new();
        if settings.server.headless {
            _ = app.add_plugins(HeadlessServerPlugins);
        } else {
            _ = app.add_plugins(ServerPlugins);
        }

        // configure the network configuration
        let mut net_configs = get_server_net_configs(settings);
        let extra_net_configs = extra_transport_configs.into_iter().map(|c| {
            build_server_netcode_config(settings.server.conditioner.as_ref(), &settings.shared, c)
        });
        net_configs.extend(extra_net_configs);
        let config = ServerConfig {
            shared: shared_config(Mode::Separate),
            net: net_configs,
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            ..default()
        };
        Self::Server { app, config }
    }

    /// An `App` that contains both the client and server plugins
    #[cfg(not(target_family = "wasm"))]
    fn combined_app(
        settings: &CommonSettings,
        extra_transport_configs: Vec<server::ServerTransport>,
        client_net_config: client::NetConfig,
    ) -> Self {
        let mut app = App::new();
        _ = app.add_plugins(ClientPlugins);

        // server config
        let mut net_configs = get_server_net_configs(settings);
        let extra_net_configs = extra_transport_configs.into_iter().map(|c| {
            build_server_netcode_config(settings.server.conditioner.as_ref(), &settings.shared, c)
        });
        net_configs.extend(extra_net_configs);
        let server_config = ServerConfig {
            shared: shared_config(Mode::HostServer),
            net: net_configs,
            replication: ReplicationConfig {
                send_interval: REPLICATION_INTERVAL,
                ..default()
            },
            ..default()
        };

        // client config
        let client_config = ClientConfig {
            shared: shared_config(Mode::HostServer),
            net: client_net_config,
            prediction: PredictionConfig {
                minimum_input_delay_ticks: settings.client.input_delay_ticks,
                maximum_predicted_ticks: settings.client.max_prediction_ticks,
                correction_ticks_factor: settings.client.correction_ticks_factor,
                ..default()
            },
            ..default()
        };
        Self::HostServer {
            app,
            server_config,
            client_config,
        }
    }
}
