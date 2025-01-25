use crate::voxel::{Voxel, VoxelGenerator};

use super::Scene;

pub struct Octree {}

impl Scene for Octree {
    fn from_voxels(generator: VoxelGenerator) -> Self {
        todo!()
    }

    fn trace(&self) -> Option<Voxel> {
        todo!()
    }
}
