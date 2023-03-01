use crate::BlockKind;
use bevy::{
    prelude::{Mesh, Resource, Vec3},
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use block_mesh::{ndshape::Shape, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility};

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub position: Vec3,
    pub index: usize,
}

#[derive(Resource)]
pub struct Chunk<S> {
    pub blocks: Box<[BlockKind]>,
    pub shape: S,
}

impl<S: Shape<3, Coord = u32>> Chunk<S> {
    pub fn filled(block: BlockKind, shape: S) -> Self {
        Self {
            blocks: vec![block; shape.usize()].into_boxed_slice(),
            shape,
        }
    }

    pub fn block(&self, position: Vec3) -> Option<Block> {
        block_index(&self.shape, position).and_then(|(position, index)| {
            self.blocks.get(index).copied().map(|kind| Block {
                kind,
                position,
                index,
            })
        })
    }

    pub fn block_mut(&mut self, position: Vec3) -> Option<&mut BlockKind> {
        block_index(&self.shape, position).and_then(|(_pos, idx)| self.blocks.get_mut(idx))
    }

    pub fn iter(&self) -> Iter<'_, S> {
        Iter {
            chunk: self,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let [x, y, z] = self.shape.as_array();

        let mut buffer = GreedyQuadsBuffer::new(self.blocks.len());
        block_mesh::greedy_quads(
            &self.blocks,
            &self.shape,
            [0; 3],
            [x - 1, y - 1, z - 1],
            &block_mesh::RIGHT_HANDED_Y_UP_CONFIG.faces,
            &mut buffer,
        );

        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        for (group, face) in buffer
            .quads
            .groups
            .into_iter()
            .zip(block_mesh::RIGHT_HANDED_Y_UP_CONFIG.faces)
        {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
        );
        mesh.set_indices(Some(Indices::U32(indices.clone())));
        mesh
    }
}

fn block_index<S>(shape: &S, position: Vec3) -> Option<(Vec3, usize)>
where
    S: Shape<3, Coord = u32>,
{
    let [x, y, z] = shape.as_array();

    if position.x < 0. || position.y < 0. || position.z < 0. {
        return None;
    }

    let rounded = position.round();
    let rounded_x = rounded.x as usize;
    let rounded_y = rounded.y as usize;
    let rounded_z = rounded.z as usize;

    if rounded_x < x as usize && rounded_z < z as usize && rounded_y < y as usize {
        let idx = rounded_x + rounded_y * y as usize + rounded_z * z as usize;
        Some((rounded, idx))
    } else {
        None
    }
}

pub struct Iter<'a, S> {
    chunk: &'a Chunk<S>,
    x: usize,
    y: usize,
    z: usize,
}

impl<S> Iterator for Iter<'_, S>
where
    S: Shape<3, Coord = u32>,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let [x, y, z] = self.chunk.shape.as_array();

        if self.y == y as usize {
            return None;
        }

        let pos = Vec3::new(self.x as _, self.y as _, self.z as _);

        if self.x == x as usize - 1 {
            self.x = 0;

            if self.z == z as usize - 1 {
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
