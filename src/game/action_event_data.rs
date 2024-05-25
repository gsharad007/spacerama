use bevy::prelude::*;

use derive_more::AddAssign;
use derive_more::Mul;

use bytemuck::{Pod, Zeroable};

#[derive(Component, Copy, Clone, Default, Debug, Mul, AddAssign, PartialEq, Pod, Zeroable)]
#[repr(C)]
// Define an event to represent the spawning of a bot
pub struct ActionEventData {
    pub thrust: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub action1: f32,
    pub action2: f32,
    pub auto_balance: f32,
}
