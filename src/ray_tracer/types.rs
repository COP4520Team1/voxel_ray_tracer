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

    /// Checks for an intersection with the bounding box.
    /// Returns the range in which the ray intersection the bounding box if so.
    /// 
    /// See: https://web.archive.org/web/20170329072729/http://www.cs.utah.edu/~awilliam/box/box.pdf
    pub fn intersection(&self, ray: Ray) -> Option<Range<f32>> {
        todo!()
    }
}
