use glam::IVec3;

use crate::voxel::{Voxel, VoxelGenerator};

use super::Scene;

pub struct Octree {}

impl Scene for Octree {
    fn from_voxels(generator: VoxelGenerator, bounds: (IVec3, IVec3)) -> Self {
        todo!()
    }

    fn trace(&self) -> Option<Voxel> {
        todo!()
    }
}
