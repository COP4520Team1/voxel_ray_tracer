use std::sync::Arc;

use crate::voxel::{Voxel, VoxelGenerator};

use super::{
    types::{IAabb, Ray},
    Scene,
};

/// This storage will be a temporary alternative to an octree until that is implemented.
pub struct DenseStorage {
    data: Arc<[Option<Voxel>]>,
    bb: IAabb,
}

impl Scene for DenseStorage {
    fn from_voxels(generator: VoxelGenerator, bb: IAabb) -> Self {
        let data = bb.iter().map(|pos| generator.lookup(pos)).collect();
        Self { data, bb }
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        // TODO: Implement fast voxel traversal algorithm (Amanatides & Woo)
        // https://github.com/cgyurgyik/fast-voxel-traversal-algorithm/blob/master/overview/FastVoxelTraversalOverview.md
        todo!()
    }
}
