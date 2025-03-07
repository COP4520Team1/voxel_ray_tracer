use std::{cmp::Ordering, sync::Arc};

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

impl DenseStorage {
    pub fn new(data: impl Into<Arc<[Option<Voxel>]>>, bb: IAabb) -> Self {
        let data = data.into();

        assert_eq!(
            data.len(),
            bb.width() * bb.height() * bb.length(),
            "aabb size was not equal to data length"
        );

        Self { data, bb }
    }
}

const VOXEL_SIZE: f32 = 1.0;

impl Scene for DenseStorage {
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        let data = bb.iter().map(|pos| generator.lookup(pos)).collect();
        Self { data, bb }
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        // See: https://github.com/cgyurgyik/fast-voxel-traversal-algorithm/blob/master/overview/FastVoxelTraversalOverview.md

        let range = self.bb.intersection(ray, 0.01..f32::INFINITY)?;

        // get end points of ray
        let ray_start = ray.origin + ray.dir * range.start;
        let ray_end = ray.origin + ray.dir * range.end;

        let min = self.bb.min().as_vec3a();

        // setup x conditions
        let mut curr_x_idx = (ray_start.x - min.x / VOXEL_SIZE).ceil().max(0.0) as usize;
        let end_x_idx = (ray_end.x - min.x / VOXEL_SIZE).ceil().max(0.0) as usize;
        let (step_x, delta_x, mut max_x) = match ray.dir.x.partial_cmp(&0.0) {
            Some(Ordering::Greater) => (
                1,
                VOXEL_SIZE / ray.dir.x,
                range.start
                    + ((min.x + curr_x_idx as f32 * VOXEL_SIZE - ray_start.x) / ray.dir.x).abs(),
            ),
            Some(Ordering::Less) => (
                -1,
                VOXEL_SIZE / -ray.dir.x,
                range.start
                    + (min.x + (curr_x_idx as f32 - 1.0) * VOXEL_SIZE - ray_start.x) / ray.dir.x,
            ),
            _ => (0, range.end, range.end),
        };

        // setup y conditions
        let mut curr_y_idx = (ray_start.y - min.y / VOXEL_SIZE).ceil().max(0.0) as usize;
        let end_y_idx = (ray_end.y - min.y / VOXEL_SIZE).ceil().max(0.0) as usize;
        let (step_y, delta_y, mut max_y) = match ray.dir.y.partial_cmp(&0.0) {
            Some(Ordering::Greater) => (
                1,
                VOXEL_SIZE / ray.dir.y,
                range.start
                    + ((min.y + curr_y_idx as f32 * VOXEL_SIZE - ray_start.y) / ray.dir.y).abs(),
            ),
            Some(Ordering::Less) => (
                -1,
                VOXEL_SIZE / -ray.dir.y,
                range.start
                    + (min.y + ((curr_y_idx as f32 - 1.0) * VOXEL_SIZE - ray_start.y) / ray.dir.y)
                        .abs(),
            ),
            _ => (0, range.end, range.end),
        };

        // setup z conditions
        let mut curr_z_idx = (ray_start.z - min.z / VOXEL_SIZE).ceil().max(0.0) as usize;
        let end_z_idx = (ray_end.z - min.z / VOXEL_SIZE).ceil().max(0.0) as usize;
        let (step_z, delta_z, mut max_z) = match ray.dir.z.partial_cmp(&0.0) {
            Some(Ordering::Greater) => (
                1,
                VOXEL_SIZE / ray.dir.z,
                range.start + (min.z + curr_z_idx as f32 * VOXEL_SIZE - ray_start.z) / ray.dir.z,
            ),
            Some(Ordering::Less) => (
                -1,
                VOXEL_SIZE / -ray.dir.z,
                range.start
                    + (min.z + (curr_z_idx as f32 - 1.0) * VOXEL_SIZE - ray_start.z) / ray.dir.z,
            ),
            _ => (0, range.end, range.end),
        };

        // use conditions to iterate over voxel spaces
        while curr_x_idx != end_x_idx || curr_y_idx != end_y_idx || curr_z_idx != end_z_idx {
            let voxel_entry = self
                .data
                .get(curr_z_idx + self.bb.width() * (curr_y_idx + self.bb.height() * curr_x_idx))?;

            if voxel_entry.is_some() {
                return *voxel_entry;
            }

            if max_x < max_y && max_x < max_z {
                curr_x_idx = curr_x_idx.saturating_add_signed(step_x);
                max_x += delta_x;
            } else if max_y < max_z {
                curr_y_idx = curr_y_idx.saturating_add_signed(step_y);
                max_y += delta_y;
            } else {
                curr_z_idx = curr_z_idx.saturating_add_signed(step_z);
                max_z += delta_z;
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use glam::{IVec3, U8Vec3, Vec3A};

    use crate::{
        ray_tracer::{
            types::{IAabb, Ray},
            Scene,
        },
        voxel::Voxel,
    };

    use super::DenseStorage;

    #[test]
    fn get_voxel_full() {
        let data = vec![Some(Voxel { color: U8Vec3::ONE }); 2 * 2 * 2];
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ONE, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(0.0, -5.0, 0.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }
    }

    #[test]
    fn get_voxel_one() {
        let mut data = vec![None; 2 * 2 * 2];
        data[0] = Some(Voxel { color: U8Vec3::ONE });
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ONE, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(0.0, -5.0, 0.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }

        {
            let ray = Ray::new(Vec3A::new(1.0, -5.0, 1.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray);
            assert_eq!(voxel, None);
        }
    }

    #[test]
    fn get_voxel_none() {
        let data = vec![None; 2 * 2 * 2];
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ONE, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(1.0, -5.0, 1.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray);
            assert_eq!(voxel, None);
        }
    }
}
