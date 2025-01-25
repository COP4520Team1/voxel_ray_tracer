use crate::{
    export::Framebuffer,
    voxel::{Voxel, VoxelGenerator},
};

pub mod dense;
pub mod octree;

pub struct RayTracer<T: Scene> {
    scene: T,
}

impl<T: Scene> RayTracer<T> {
    pub fn from_voxels(generator: VoxelGenerator) -> Self {
        Self {
            scene: T::from_voxels(generator),
        }
    }

    pub async fn render(&self) -> Framebuffer {
        todo!()
    }
}

/// A scene is a data structure for the voxel data.
///
/// Since there is overlap between the data structures,
/// we can abstract the functionality into a trait.
pub trait Scene {
    /// Collects voxels from a generator.
    fn from_voxels(generator: VoxelGenerator) -> Self;

    /// TODO: change type signature to include parameters needed.
    fn trace(&self) -> Option<Voxel>;
}
