use std::sync::Arc;

use glam::{IVec3, UVec3};

use crate::voxel::{Voxel, VoxelGenerator};

use super::Scene;

/// This storage will be a temporary alternative to an octree until that is implemented.
pub struct DenseStorage {
    data: Arc<[Option<Voxel>]>,
    dims: UVec3,
}

impl Scene for DenseStorage {
    fn from_voxels(generator: VoxelGenerator, bounds: (IVec3, IVec3)) -> Self {
        todo!()
    }

    fn trace(&self) -> Option<Voxel> {
        todo!()
    }
}
