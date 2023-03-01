use crate::BlockKind;
use bevy::prelude::{Resource, Vec3};

const CHUNK_SIZE: usize = 4;
const CHUNK_DEPTH: usize = 8;
const CHUNK_BLOCKS_LEN: usize = CHUNK_SIZE.pow(2) * CHUNK_DEPTH;

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub position: Vec3,
    pub index: usize,
}

#[derive(Resource)]
pub struct Chunk {
    pub blocks: [BlockKind; CHUNK_BLOCKS_LEN],
}

impl Chunk {
    pub const fn filled(block: BlockKind) -> Self {
        Self {
            blocks: [block; CHUNK_BLOCKS_LEN],
        }
    }

    pub fn block(&self, position: Vec3) -> Option<Block> {
        block_index(position).and_then(|(position, index)| {
            self.blocks.get(index).copied().map(|kind| Block {
                kind,
                position,
                index,
            })
        })
    }

    pub fn block_mut(&mut self, position: Vec3) -> Option<&mut BlockKind> {
        block_index(position).and_then(|(_pos, idx)| self.blocks.get_mut(idx))
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            chunk: self,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

fn block_index(position: Vec3) -> Option<(Vec3, usize)> {
    if position.x < 0. || position.y < 0. || position.z < 0. {
        return None;
    }

    let rounded = position.round();
    let x = rounded.x as usize;
    let y = rounded.y as usize;
    let z = rounded.z as usize;

    if x < CHUNK_SIZE && z < CHUNK_SIZE && y < CHUNK_DEPTH {
        let idx = x + y * CHUNK_SIZE + z * CHUNK_DEPTH;
        Some((rounded, idx))
    } else {
        None
    }
}

pub struct Iter<'a> {
    chunk: &'a Chunk,
    x: usize,
    y: usize,
    z: usize,
}

impl Iterator for Iter<'_> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == CHUNK_DEPTH - 1 {
            return None;
        }

        let pos = Vec3::new(self.x as _, self.y as _, self.z as _);

        if self.x == CHUNK_SIZE - 1 {
            self.x = 0;

            if self.z == CHUNK_SIZE - 1 {
                self.z = 0;
                self.y += 1;
            } else {
                self.z += 1;
            }
        } else {
            self.x += 1;
        }

        self.chunk.block(pos)
    }
}
