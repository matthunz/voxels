use bevy::prelude::Vec3;

use crate::Block;

const CHUNK_SIZE: usize = 4;
const CHUNK_DEPTH: usize = 8;
const CHUNK_BLOCKS_LEN: usize = CHUNK_SIZE.pow(2) * CHUNK_DEPTH;

pub struct Chunk {
    blocks: [Block; CHUNK_BLOCKS_LEN],
}

impl Chunk {
    pub const fn filled(block: Block) -> Self {
        Self {
            blocks: [block; CHUNK_BLOCKS_LEN],
        }
    }

    pub fn block(&self, position: Vec3) -> Option<Block> {
        self.blocks.get(block_index(position)).copied()
    }

    pub fn block_mut(&mut self, position: Vec3) -> Option<&mut Block> {
        self.blocks.get_mut(block_index(position))
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

fn block_index(position: Vec3) -> usize {
    position.x.round() as usize
        + position.y.round() as usize * CHUNK_SIZE
        + position.z.round() as usize * CHUNK_DEPTH
}

pub struct Iter<'a> {
    chunk: &'a Chunk,
    x: usize,
    y: usize,
    z: usize,
}

impl Iterator for Iter<'_> {
    type Item = (Vec3, Block);

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

        Some((pos, self.chunk.block(pos).unwrap()))
    }
}
