use bevy::prelude::Component;

mod block;
pub use block::{Block, BlockKind};

pub mod chunk;
pub use chunk::Chunk;

pub mod player;

#[derive(Component)]
pub struct Selection;
