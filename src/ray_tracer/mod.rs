use types::{IAabb, Ray};

use crate::{
    export::Framebuffer,
    voxel::{Voxel, VoxelGenerator},
};

pub mod dense;
pub mod octree;
pub mod types;

pub struct RayTracer<T: Scene> {
    scene: T,
}

impl<T: Scene> RayTracer<T> {
    /// Creates a ray tracer with a scene from a voxel generator and bounds.
    pub fn from_voxels(generator: VoxelGenerator, bb: IAabb) -> Self {
        Self {
            scene: T::from_voxels(generator, bb),
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
    fn from_voxels(generator: VoxelGenerator, bb: IAabb) -> Self;

    /// Trace a ray into the scene to get voxel information.
    /// TODO: Should it return voxel or pixel info?
    fn trace(&self, ray: Ray) -> Option<Voxel>;
}
