use std::ops::Range;

use glam::{BVec3, IVec3, Vec3A};
use itertools::Itertools;

/// Ray-casting primitive.
#[derive(Clone, Copy)]
pub struct Ray {
    /// Starting position of the ray.
    pub origin: Vec3A,
    /// Direction of the ray (normalized).
    pub dir: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, dir: Vec3A) -> Self {
        Self {
            origin,
            dir: dir.normalize(),
        }
    }
}

/// Signed-integer axis-aligned bounding box.
#[derive(Clone, Copy)]
pub struct IAabb {
    /// Position of AABB.
    pub origin: IVec3,
    /// Half extents surrounding `origin` (must be positive).
    pub extents: IVec3,
}

impl IAabb {
    /// Create a new bounding box.
    pub fn new(origin: IVec3, extents: IVec3) -> Self {
        assert!(extents.cmpgt(IVec3::ZERO) == BVec3::TRUE);

        Self { origin, extents }
    }

    /// Width of box (x).
    pub fn width(&self) -> usize {
        (self.extents.x * 2) as usize
    }

    /// Width of box (x).
    pub fn height(&self) -> usize {
        (self.extents.y * 2) as usize
    }

    /// Width of box (x).
    pub fn length(&self) -> usize {
        (self.extents.z * 2) as usize
    }

    /// Iterate over values in x-axis.
    pub fn iter_x(&self) -> Range<i32> {
        (self.origin.x - self.extents.x)..(self.origin.x + self.extents.x)
    }

    /// Iterate over values in y-axis.
    pub fn iter_y(&self) -> Range<i32> {
        (self.origin.y - self.extents.y)..(self.origin.y + self.extents.y)
    }

    /// Iterate over values in z-axis.
    pub fn iter_z(&self) -> Range<i32> {
        (self.origin.z - self.extents.z)..(self.origin.z + self.extents.z)
    }

    /// Iterate over cartesian product of axes.
    pub fn iter(&self) -> impl Iterator<Item = IVec3> {
        self.iter_x()
            .cartesian_product(self.iter_y())
            .cartesian_product(self.iter_z())
            .map(|((x, y), z)| IVec3::new(x, y, z))
    }

    /// Get minimum point in bounds.
    pub fn min(&self) -> IVec3 {
        self.origin - self.extents
    }

    /// Get maximum point in bounds.
    pub fn max(&self) -> IVec3 {
        self.origin + self.extents
    }

    /// Checks for an intersection with the bounding box along a range of a ray.
    /// Returns the range in which the ray intersects the bounding box if so.
    ///
    /// See: https://web.archive.org/web/20170329072729/http://www.cs.utah.edu/~awilliam/box/box.pdf
    pub fn intersection(&self, ray: Ray, range: Range<f32>) -> Option<Range<f32>> {
        let min = self.min().as_vec3a();
        let max = self.max().as_vec3a();

        let x_dir_inv = 1.0 / ray.dir.x;

        let (x_min, x_max) = if x_dir_inv >= 0.0 {
            (
                (min.x - ray.origin.x) * x_dir_inv,
                (max.x - ray.origin.x) * x_dir_inv,
            )
        } else {
            (
                (max.x - ray.origin.x) * x_dir_inv,
                (min.x - ray.origin.x) * x_dir_inv,
            )
        };

        let y_dir_inv = 1.0 / ray.dir.y;

        let (y_min, y_max) = if y_dir_inv >= 0.0 {
            (
                (min.y - ray.origin.y) * y_dir_inv,
                (max.y - ray.origin.y) * y_dir_inv,
            )
        } else {
            (
                (max.y - ray.origin.y) * y_dir_inv,
                (min.y - ray.origin.y) * y_dir_inv,
            )
        };

        if x_min > y_max || y_min > x_max {
            return None;
        }

        let (t_min, t_max) = (x_min.max(y_min), x_max.min(y_max));

        let z_dir_inv = 1.0 / ray.dir.z;

        let (z_min, z_max) = if z_dir_inv >= 0.0 {
            (
                (min.z - ray.origin.z) * z_dir_inv,
                (max.z - ray.origin.z) * z_dir_inv,
            )
        } else {
            (
                (max.z - ray.origin.z) * z_dir_inv,
                (min.z - ray.origin.z) * z_dir_inv,
            )
        };

        if t_min > z_max || z_min > t_max {
            return None;
        }

        let (start, end) = (t_min.max(z_min), t_max.min(z_max));

        if start > range.end || end < range.start {
            return None;
        }

        Some(start..end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Check for an intersection.
    fn intersects() {
        let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

        assert!(bb
            .intersection(
                Ray::new(Vec3A::NEG_ONE * 6.0, Vec3A::ONE),
                0.0..f32::INFINITY
            )
            .is_some());
    }

    #[test]
    /// Check for no intersection.
    fn not_intersects() {
        let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

        assert!(bb
            .intersection(Ray::new(Vec3A::ONE * 6.0, Vec3A::ONE), 0.0..f32::INFINITY)
            .is_none());
    }
}
