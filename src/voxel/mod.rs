use glam::{IVec3, U8Vec3};

/// Data associated with a single voxel.
pub struct Voxel {
    pub color: U8Vec3,
}

/// An iterator that produces voxels.
pub struct VoxelGenerator {}

impl Iterator for VoxelGenerator {
    type Item = (IVec3, Voxel);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Create a new voxel generator
pub fn generate_voxels() -> VoxelGenerator {
    todo!();
}
