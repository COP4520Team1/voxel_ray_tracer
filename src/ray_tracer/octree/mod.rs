use crate::voxel::{Voxel, VoxelGenerator};

use super::{
    types::{IAabb, Ray},
    Scene,
};

pub struct Octree {}

impl Scene for Octree {
    fn from_voxels(generator: VoxelGenerator, bb: IAabb) -> Self {
        todo!()
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        todo!()
    }
}
