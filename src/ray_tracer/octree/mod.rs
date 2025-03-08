use crate::voxel::{Voxel, VoxelGenerator};

use super::{
    types::{IAabb, Ray},
    Scene,
};

pub struct SparseStorage {
    octree: Octree,
}

impl Scene for SparseStorage {
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        todo!()
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        todo!()
    }
}

pub struct Octree {}

pub struct Node {
    pub vox: Voxel,
    pub children: [Option<Box<Node>>; 8],
}

/// Node is what stores the voxel and its children
///
impl Node {
    fn new(vox: Voxel) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            vox,
        }
    }
    fn insert(&mut self, index: usize, vox: Voxel) {
        if index < 8 {
            self.children[index] = Some(Box::new(Node::new(vox)));
        }
    }
}