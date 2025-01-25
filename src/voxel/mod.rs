use glam::{U8Vec3, UVec3};

/// Data associated with a single voxel.
pub struct Voxel {
    pub color: U8Vec3,
}

/// An iterator that produces voxels.
pub struct VoxelGenerator {}

impl VoxelGenerator {
    /// Provides a hint about the dimensions of the voxel space.
    pub fn dims() -> UVec3 {
        todo!()
    }
}

impl Iterator for VoxelGenerator {
    type Item = Option<Voxel>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Create a new voxel generator
pub fn generate_voxels() -> VoxelGenerator {
    todo!();
}
