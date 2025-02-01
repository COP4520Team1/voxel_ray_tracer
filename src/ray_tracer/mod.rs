use glam::IVec3;

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
    /// Creates a ray tracer with a scene from a voxel generator and bounds.
    ///
    /// `bounds` represents the `(lower, upper)` bounds that voxels are generated in, where for each dimension1 `lower < higher`.
    pub fn from_voxels(generator: VoxelGenerator, bounds: (IVec3, IVec3)) -> Self {
        Self {
            scene: T::from_voxels(generator, bounds),
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
    ///
    /// `bounds` represents the `(lower, upper)` bounds that voxels are generated in, where for each dimension1 `lower < higher`.
    fn from_voxels(generator: VoxelGenerator, bounds: (IVec3, IVec3)) -> Self;

    /// TODO: change type signature to include parameters needed.
    fn trace(&self) -> Option<Voxel>;
}
