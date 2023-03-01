use bevy::prelude::Vec3;
use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Air,
    Grass,
}

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub position: Vec3,
    pub index: usize,
}

impl Voxel for BlockKind {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == BlockKind::Air {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BlockKind {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}
