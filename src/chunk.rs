use crate::BlockKind;
use bevy::{
    prelude::{Mesh, Resource, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use block_mesh::{
    ndshape::{ConstShape, ConstShape3u32},
    Axis, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility,
};

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

    pub fn mesh(&self) -> Mesh {
        let mut buffer = GreedyQuadsBuffer::new(self.blocks.len());
        block_mesh::greedy_quads(
            &self.blocks,
            &ChunkShape {},
            [0; 3],
            [
                CHUNK_SIZE as u32 - 1,
                CHUNK_DEPTH as u32 - 1,
                CHUNK_SIZE as u32 - 1,
            ],
            &block_mesh::RIGHT_HANDED_Y_UP_CONFIG.faces,
            &mut buffer,
        );

        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        let mut uvs = Vec::with_capacity(num_vertices);

        for (block_face_normal_index, (group, face)) in buffer
            .quads
            .groups
            .as_ref()
            .into_iter()
            .zip(block_mesh::RIGHT_HANDED_Y_UP_CONFIG.faces)
            .enumerate()
        {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.));
                normals.extend_from_slice(&face.quad_mesh_normals());
                uvs.extend_from_slice(&face.tex_coords(Axis::Y, false, quad));
            }
        }

        dbg!(positions.len(), normals.len(), indices.len());

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
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

// A 16^3 chunk with 1-voxel boundary padding.
type ChunkShape =
    ConstShape3u32<{ CHUNK_SIZE as u32 }, { CHUNK_DEPTH as u32 }, { CHUNK_SIZE as u32 }>;
