use std::sync::atomic::Ordering;

use glam::{IVec3, Vec3A};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use types::{IAabb, Ray};

#[cfg(feature = "trace")]
use tracing::*;

use crate::{
    camera::Camera,
    export::{Framebuffer, PixelRef},
    voxel::{Voxel, VoxelGenerator},
};

pub mod dense;
pub mod octree;
pub mod types;

pub struct RayTracer<T: Scene + Sync> {
    config: Config,
    scene: T,
    camera: Camera,
}

impl<T: Scene + Sync> RayTracer<T> {
    /// Creates a ray tracer from a config.
    pub fn new(config: Config) -> Self {
        #[cfg(feature = "trace")]
        let _span = trace_span!("ray_tracer_new").entered();

        let bb = IAabb::new(IVec3::ZERO, config.size as i32 * IVec3::ONE);
        let generator = config
            .seed
            .map(VoxelGenerator::new_from_seed)
            .unwrap_or_else(VoxelGenerator::new);

        Self {
            config,
            scene: T::from_voxels(&generator, bb),
            camera: Camera::from_res_and_pos(
                config.res_width,
                config.res_height,
                config.camera_pos,
            ),
        }
    }

    pub fn render(&self) -> Framebuffer {
        #[cfg(feature = "trace")]
        let _span = trace_span!("ray_tracer_render").entered();

        let fb = Framebuffer::new(self.config.res_width, self.config.res_height);

        fb.into_par_iter().for_each(|pixel| {
            self.render_pixel(pixel);
        });

        fb
    }

    fn render_pixel(&self, pixel: PixelRef<'_>) {
        #[cfg(feature = "trace")]
        let _span = trace_span!("ray_tracer_render_pixel").entered();

        let ray = self.camera.get_ray(pixel.x, pixel.y);

        let Some(voxel) = self.scene.trace(ray, self.config.debug) else {
            return;
        };

        let raw_color = voxel.color.as_uvec3();
        let color = raw_color.x << 24 | raw_color.y << 16 | raw_color.z << 8 | 0xff;

        pixel.value.store(color, Ordering::Release);
    }
}

#[derive(Debug, Clone, Copy)]
/// Ray tracer configuration.
pub struct Config {
    pub seed: Option<u32>,
    pub size: u32,
    pub camera_pos: Vec3A,
    pub res_width: usize,
    pub res_height: usize,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seed: None,
            size: 100,
            camera_pos: 100.0 * Vec3A::ONE,
            res_width: 1920,
            res_height: 1080,
            debug: false,
        }
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
    ///
    /// `debug` flag enables an alternative debug render mode, if available.
    fn trace(&self, ray: Ray, debug: bool) -> Option<Voxel>;
}
