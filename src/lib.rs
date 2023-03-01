use bevy::prelude::Component;

pub mod chunk;
pub use chunk::Chunk;

pub mod player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Air,
    Grass,
}

#[derive(Component)]
pub struct Selection;
