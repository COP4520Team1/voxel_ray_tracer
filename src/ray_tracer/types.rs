use std::ops::Range;

use glam::{BVec2, BVec3, IVec3, Vec3A, Vec3Swizzles};
use itertools::Itertools;

/// Ray-casting primitive.
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Returns the next power of two extent.
    ///
    /// Takes the maximum dimension and uses that to make a cube with sides that are a power of two.
    pub fn next_pow2(&self) -> Self {
        let max_extent = self.extents.max_element() as u32;

        // bit twiddling to next power of two
        let mut pow2 = max_extent;
        pow2 |= pow2 >> 1;
        pow2 |= pow2 >> 2;
        pow2 |= pow2 >> 4;
        pow2 |= pow2 >> 8;
        pow2 |= pow2 >> 16;
        pow2 += 1;

        let pow2 = pow2 as i32;

        Self {
            origin: self.origin,
            extents: pow2 * IVec3::ONE,
        }
    }

    /// Finds the index for the octant that the position lies in.
    pub fn index_of(&self, pos: IVec3) -> Option<usize> {
        // we can move the position into local space and compare the sign
        let local_pos = pos - self.origin;

        // calculate index
        let positive = local_pos.cmpgt(IVec3::ZERO);
        let x = positive.x as i32;
        let y = positive.y as i32;
        let z = positive.z as i32;
        let xyz = (x | y << 1 | z << 2) as usize;

        // make the position zero exclusive to simplify bounds check (so dist(0) == dist(1), since 0.5 is the middle but we work with integers)
        let local_pos = local_pos - IVec3::new(1 - x, 1 - y, 1 - z);

        // check that local position is inside extents
        if local_pos.abs().cmple(self.extents) != BVec3::TRUE {
            return None;
        }

        Some(xyz)
    }

    /// Gets the octant AABB from an index.
    ///
    /// Panics if the idx is outside of range (greater than 0b111).
    pub fn octant(&self, idx: usize) -> Self {
        assert!(idx <= 0b111, "index is out of range");
        let idx = idx as i32;

        // calculate bounding box
        let extents = self.extents / 2;

        let offset = IVec3 {
            x: (idx & 0b001) * 2 - 1,
            y: ((idx & 0b010) >> 1) * 2 - 1,
            z: ((idx & 0b100) >> 2) * 2 - 1,
        };
        let origin = self.origin + (extents * offset);

        Self { origin, extents }
    }

    /// Checks if the longest side is one.
    pub fn is_unit(&self) -> bool {
        self.extents.max_element() == 1
    }

    /// Checks if a ray intersects the edge of the bounding box.
    pub fn intersects_edge(&self, ray: Ray) -> bool {
        let min = self.min().as_vec3a();
        let max = self.max().as_vec3a();

        let dir_x = ray.dir.dot(Vec3A::X);
        let dir_y = ray.dir.dot(Vec3A::Y);
        let dir_z = ray.dir.dot(Vec3A::Z);

        fn cmp_points(a: Option<Vec3A>, b: Option<Vec3A>) -> bool {
            const MAX_DIST: f32 = 0.1;
            a.zip(b)
                .filter(|(i, j)| i.distance_squared(*j) < MAX_DIST)
                .is_some()
        }

        let min_x = if dir_x != 0.0 {
            let t = (min.dot(Vec3A::X) - ray.origin.dot(Vec3A::X)) / dir_x;
            let p = ray.origin + t * ray.dir;
            let i = p.yz();
            if i.cmpge(min.yz()) == BVec2::TRUE && i.cmple(max.yz()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        let min_y = if dir_y != 0.0 {
            let t = (min.dot(Vec3A::Y) - ray.origin.dot(Vec3A::Y)) / dir_y;
            let p = ray.origin + t * ray.dir;
            let i = p.xz();
            if i.cmpge(min.xz()) == BVec2::TRUE && i.cmple(max.xz()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        if cmp_points(min_x, min_y) {
            return true;
        }

        let min_z = if dir_z != 0.0 {
            let t = (min.dot(Vec3A::Z) - ray.origin.dot(Vec3A::Z)) / dir_z;
            let p = ray.origin + t * ray.dir;
            let i = p.xy();
            if i.cmpge(min.xy()) == BVec2::TRUE && i.cmple(max.xy()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        if min_z.is_some() && (cmp_points(min_z, min_y) || cmp_points(min_z, min_x)) {
            return true;
        }

        let max_x = if dir_x != 0.0 {
            let t = (max.dot(Vec3A::X) - ray.origin.dot(Vec3A::X)) / dir_x;
            let p = ray.origin + t * ray.dir;
            let i = p.yz();
            if i.cmpge(min.yz()) == BVec2::TRUE && i.cmple(max.yz()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        if max_x.is_some() && (cmp_points(max_x, min_z) || cmp_points(max_x, min_y)) {
            return true;
        }

        let max_y = if dir_y != 0.0 {
            let t = (max.dot(Vec3A::Y) - ray.origin.dot(Vec3A::Y)) / dir_y;
            let p = ray.origin + t * ray.dir;
            let i = p.xz();
            if i.cmpge(min.xz()) == BVec2::TRUE && i.cmple(max.xz()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        if max_y.is_some()
            && (cmp_points(max_y, max_x) || cmp_points(max_y, min_z) || cmp_points(max_y, min_x))
        {
            return true;
        }

        let max_z = if dir_z != 0.0 {
            let t = (max.dot(Vec3A::Z) - ray.origin.dot(Vec3A::Z)) / dir_z;
            let p = ray.origin + t * ray.dir;
            let i = p.xy();
            if i.cmpge(min.xy()) == BVec2::TRUE && i.cmple(max.xy()) == BVec2::TRUE {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        max_z.is_some()
            && (cmp_points(max_z, max_y)
                || cmp_points(max_z, max_x)
                || cmp_points(max_z, min_y)
                || cmp_points(max_z, min_x))
    }

    /// Checks for ray intersections across planes inside of the bounding box.
    /// Returns the distance if found.
    ///
    /// (x, y, z) axes -> (yz, xz, xy) planes
    pub fn plane_intersections(&self, ray: Ray) -> [Option<f32>; 3] {
        let min = self.min().as_vec3a();
        let max = self.max().as_vec3a();

        let origin = self.origin.as_vec3a();

        let dir_x = ray.dir.dot(Vec3A::X);
        let x = if dir_x != 0.0
            && dir_x.is_sign_positive() == (origin.x - ray.origin.x).is_sign_positive()
        {
            let t = (origin.dot(Vec3A::X) - ray.origin.dot(Vec3A::X)) / dir_x;
            let i = ray.origin + t * ray.dir;
            let p = i.yz();
            if p.cmpge(min.yz()) == BVec2::TRUE && p.cmple(max.yz()) == BVec2::TRUE {
                Some(i.distance(ray.origin))
            } else {
                None
            }
        } else {
            None
        };

        let dir_y = ray.dir.dot(Vec3A::Y);
        let y = if dir_y != 0.0
            && dir_y.is_sign_positive() == (origin.y - ray.origin.y).is_sign_positive()
        {
            let t = (origin.dot(Vec3A::Y) - ray.origin.dot(Vec3A::Y)) / dir_y;
            let i = ray.origin + t * ray.dir;
            let p = i.xz();
            if p.cmpge(min.xz()) == BVec2::TRUE && p.cmple(max.xz()) == BVec2::TRUE {
                Some(i.distance(ray.origin))
            } else {
                None
            }
        } else {
            None
        };

        let dir_z = ray.dir.dot(Vec3A::Z);
        let z = if dir_z != 0.0
            && dir_z.is_sign_positive() == (origin.z - ray.origin.z).is_sign_positive()
        {
            let t = (origin.dot(Vec3A::Z) - ray.origin.dot(Vec3A::Z)) / dir_z;
            let i = ray.origin + t * ray.dir;
            let p = i.xy();
            if p.cmpge(min.xy()) == BVec2::TRUE && p.cmple(max.xy()) == BVec2::TRUE {
                Some(i.distance(ray.origin))
            } else {
                None
            }
        } else {
            None
        };

        [x, y, z]
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

    #[test]
    /// Check for plane intersections.
    fn planes_intersect() {
        {
            let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

            assert!(matches!(
                bb.plane_intersections(Ray::new(Vec3A::NEG_ONE * 6.0, Vec3A::ONE)),
                [Some(_), Some(_), Some(_)]
            ));
        }

        {
            let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

            assert!(matches!(
                bb.plane_intersections(Ray::new(Vec3A::NEG_ONE * 2.0, Vec3A::Y)),
                [None, Some(_), None]
            ));
        }

        {
            let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

            assert!(matches!(
                bb.plane_intersections(Ray::new(Vec3A::ONE * 2.0, Vec3A::NEG_Y)),
                [None, Some(_), None]
            ));
        }
    }

    #[test]
    /// Check for plane intersections.
    fn planes_not_intersect() {
        {
            let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

            assert!(matches!(
                bb.plane_intersections(Ray::new(Vec3A::NEG_ONE * 6.0, Vec3A::Y)),
                [None, None, None]
            ));
        }

        {
            let bb = IAabb::new(IVec3::ZERO, IVec3::ONE * 5);

            assert!(matches!(
                bb.plane_intersections(Ray::new(Vec3A::NEG_Y * 6.0, Vec3A::NEG_Y)),
                [None, None, None]
            ));
        }
    }

    #[test]
    /// Check for correct indices.
    fn indices() {
        let bb = IAabb::new(IVec3::ZERO, 2 * IVec3::ONE);

        assert_eq!(bb.index_of(2 * IVec3::NEG_ONE), None);
        assert_eq!(bb.index_of(IVec3::NEG_ONE), Some(0b000));
        assert_eq!(bb.index_of(IVec3::ZERO), Some(0b000));
        assert_eq!(bb.index_of(IVec3::ONE), Some(0b111));
        assert_eq!(bb.index_of(2 * IVec3::ONE), Some(0b111));
        assert_eq!(bb.index_of(3 * IVec3::ONE), None);
    }

    #[test]
    fn octants() {
        let bb = IAabb::new(IVec3::ZERO, 2 * IVec3::ONE);
        assert_eq!(bb.octant(0b000), IAabb::new(IVec3::NEG_ONE, IVec3::ONE));
        assert_eq!(bb.octant(0b111), IAabb::new(IVec3::ONE, IVec3::ONE));
    }
}
