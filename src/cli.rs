use std::fmt::{Debug, Display, Formatter, Result};

use bevy::ecs::system::Resource;

use clap::Parser;
use rand::RngCore;

#[derive(Parser, Resource, Clone, Debug)]
#[command(version, about, long_about = None)]
pub enum CommandLineArguments {
    /// We have the client and the server running inside the same app.
    /// The server will also act as a client. (i.e. one client acts as the 'host')
    #[cfg(not(target_family = "wasm"))]
    HostServer {
        #[arg(short, long, default_value_t = random_client_id())]
        client_id: u64,
    },
    /// We will create two apps: a client app and a server app.
    /// Data gets passed between the two via channels.
    #[cfg(not(target_family = "wasm"))]
    ServerAndClient {
        #[arg(short, long, default_value_t = random_client_id())]
        client_id: u64,
    },
    /// Dedicated server
    #[cfg(not(target_family = "wasm"))]
    Server,
    /// The program will act as a client
    Client {
        #[arg(short, long, default_value_t = random_client_id())]
        client_id: u64,
    },
}

// Implementing Default for CommandLineArguments
impl Default for CommandLineArguments {
    fn default() -> Self {
        #[cfg(target_family = "wasm")]
        {
            // Default to `Client` on wasm target
            Self::Client {
                client_id: random_client_id(),
            }
        }
        {
            // Default to `HostServer` to make it default to standalone game
            Self::HostServer {
                client_id: random_client_id(),
            }
        }
    }
}

// Implementing Display for CommandLineArguments
impl Display for CommandLineArguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

// Function to generate a random client_id using bevy_rand
fn random_client_id() -> u64 {
    rand::thread_rng().next_u64()
}
