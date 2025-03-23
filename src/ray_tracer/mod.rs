use std::sync::atomic::{AtomicU32, Ordering};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use types::{IAabb, Ray};

use crate::{
    camera::Camera,
    export::Framebuffer,
    voxel::{Voxel, VoxelGenerator},
};

pub mod dense;
pub mod octree;
pub mod types;

pub struct RayTracer<T: Scene + Sync> {
    scene: T,
    camera: Camera,
}

impl<T: Scene + Sync> RayTracer<T> {
    /// Creates a ray tracer with a scene from a voxel generator and bounds.
    pub fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        Self {
            scene: T::from_voxels(generator, bb),
            camera: Camera::default(),
        }
    }

    pub fn render(&self) -> Framebuffer {
        let fb = Framebuffer::new(self.camera.img_width, self.camera.img_height);

        fb.into_par_iter().for_each(|pixel| {
            let ray = self.camera.get_ray(pixel.x, pixel.y);
            self.render_pixel(pixel.value, ray);
        });

        fb
    }

    fn render_pixel(&self, pixel: &AtomicU32, ray: Ray) {
        let Some(voxel) = self.scene.trace(ray) else {
            return;
        };

        let raw_color = voxel.color.as_uvec3();
        let color = raw_color.x << 24 | raw_color.y << 16 | raw_color.z << 8 | 0xff;

        pixel.store(color, Ordering::Release);
    }
}

/// A scene is a data structure for the voxel data.
///
/// Since there is overlap between the data structures,
/// we can abstract the functionality into a trait.
pub trait Scene {
    /// Collects voxels from a generator.
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self;

    /// Trace a ray into the scene to get voxel information.
    /// TODO: Should it return voxel or pixel info?
    fn trace(&self, ray: Ray) -> Option<Voxel>;
}
