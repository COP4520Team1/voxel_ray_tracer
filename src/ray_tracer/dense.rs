use std::sync::Arc;

use glam::Vec3A;

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

/// Size of a voxel in the grid.
const VOXEL_SIZE: f32 = 1.0;

/// Maximum number of steps to try to get a voxel in the grid.
///
/// TODO: Should we have a maximum, when we can just test out of bounds?
const MAX_STEPS: usize = 256;

impl Scene for DenseStorage {
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        let data = bb.iter().map(|pos| generator.lookup(pos)).collect();
        Self { data, bb }
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        // See (for basic impl): https://github.com/cgyurgyik/fast-voxel-traversal-algorithm/blob/master/overview/FastVoxelTraversalOverview.md
        // See (for DRY impl): https://m4xc.dev/articles/amanatides-and-woo/

        let range = self.bb.intersection(ray, 0.01..f32::INFINITY)?;

        let ray_start = ray.origin + ray.dir * (range.start + 0.0001);

        let max = self.bb.max().as_vec3a();
        let min = self.bb.min().as_vec3a();

        let entry_pos = (ray_start - min) * VOXEL_SIZE;

        let step = ray.dir.signum();
        let delta = (1.0 / ray.dir).abs();

        let pos = entry_pos.floor().clamp(Vec3A::ZERO, max - min);

        let mut tmax = (pos - entry_pos + step.max(Vec3A::ZERO)) / ray.dir;

        let mut curr_idx = pos.as_ivec3();

        let step = step.as_ivec3();

        // use conditions to iterate over voxel spaces
        for _ in 0..MAX_STEPS {
            let voxel_entry = self.data.get(
                curr_idx.z as usize
                    + self.bb.width()
                        * (curr_idx.y as usize + self.bb.height() * curr_idx.x as usize),
            )?;

            if voxel_entry.is_some() {
                return *voxel_entry;
            }

            if tmax.x < tmax.y && tmax.x < tmax.z {
                curr_idx.x += step.x;
                if curr_idx.x < 0 {
                    break;
                }
                tmax.x += delta.x;
            } else if tmax.y < tmax.z {
                curr_idx.y += step.y;
                if curr_idx.y < 0 {
                    break;
                }
                tmax.y += delta.y;
            } else {
                curr_idx.z += step.z;
                if curr_idx.z < 0 {
                    break;
                }
                tmax.z += delta.z;
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
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(0.0, -5.0, 0.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }
    }

    #[test]
    fn get_voxel_one() {
        let mut data = vec![None; 2 * 2 * 2];
        data[0] = Some(Voxel { color: U8Vec3::ONE });
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(-0.5, -5.0, -0.5), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, -5.0, 0.5), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray);
            assert_eq!(voxel, None);
        }
    }

    #[test]
    fn get_voxel_none() {
        let data = vec![None; 2 * 2 * 2];
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(1.0, -5.0, 1.0), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray);
            assert_eq!(voxel, None);
        }
    }

    #[test]
    fn get_voxel_dirs() {
        let data = vec![
            Some(Voxel {
                color: U8Vec3::new(0, 0, 0),
            }),
            Some(Voxel {
                color: U8Vec3::new(0, 0, 1),
            }),
            Some(Voxel {
                color: U8Vec3::new(0, 1, 0),
            }),
            Some(Voxel {
                color: U8Vec3::new(0, 1, 1),
            }),
            Some(Voxel {
                color: U8Vec3::new(1, 0, 0),
            }),
            Some(Voxel {
                color: U8Vec3::new(1, 0, 1),
            }),
            Some(Voxel {
                color: U8Vec3::new(1, 1, 0),
            }),
            Some(Voxel {
                color: U8Vec3::new(1, 1, 1),
            }),
        ];
        let storage = DenseStorage::new(data, IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(-0.5, -5.0, -0.5), Vec3A::Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 0, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-5.0, -0.5, 0.5), Vec3A::X);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 0, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-0.5, 5.0, -0.5), Vec3A::NEG_Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 1, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-0.5, 5.0, 0.5), Vec3A::NEG_Y);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 1, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(5.0, -0.5, -0.5), Vec3A::NEG_X);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 0, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(5.0, -0.5, 0.5), Vec3A::NEG_X);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 0, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, 0.5, -5.0), Vec3A::Z);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 1, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, 0.5, 5.0), Vec3A::NEG_Z);
            assert!(storage.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = storage.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 1, 1)
                }
            );
        }
    }
}
