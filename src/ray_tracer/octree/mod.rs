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
