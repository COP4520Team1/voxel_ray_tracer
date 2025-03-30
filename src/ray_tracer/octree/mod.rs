use std::fmt;

use glam::IVec3;

use crate::voxel::{Voxel, VoxelGenerator};

use super::{
    types::{IAabb, Ray},
    Scene,
};

pub struct SparseStorage {
    octree: Octree,
}

impl Scene for SparseStorage {
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        let octree = Octree::from_voxels(generator, bb);
        println!("Length: {}", octree.len());
        Self { octree }
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        self.octree.trace(ray)
    }
}

/// Simple octree implementation with fixed size.
pub struct Octree {
    bb: IAabb,
    nodes: Vec<Node>,
}

impl fmt::Debug for Octree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut set = f.debug_set();

        fn fmt_node(
            idx: usize,
            bb: IAabb,
            nodes: &[Node],
            set: &mut fmt::DebugSet<'_, '_>,
        ) -> fmt::Result {
            match nodes[idx] {
                Node::Branch(branches) => {
                    for z in 0..2 {
                        for y in 0..2 {
                            for x in 0..2 {
                                let local_idx = x | y << 1 | z << 2;
                                let Some(next_idx) = branches[local_idx] else {
                                    continue;
                                };
                                let next_bb = bb.octant(local_idx);
                                fmt_node(next_idx, next_bb, nodes, set)?;
                            }
                        }
                    }
                }
                Node::Leaf(leaves) => {
                    for z in 0..2 {
                        for y in 0..2 {
                            for x in 0..2 {
                                let local_idx = x | y << 1 | z << 2;
                                let Some(leaf) = leaves[local_idx] else {
                                    continue;
                                };
                                let pos = IVec3::new(
                                    x as i32 + bb.origin.x,
                                    y as i32 + bb.origin.x,
                                    z as i32 + bb.origin.x,
                                );
                                set.entry(&(pos, leaf));
                            }
                        }
                    }
                }
            }

            Ok(())
        }

        fmt_node(0, self.bb, &self.nodes, &mut set)?;

        set.finish()
    }
}

impl Octree {
    pub fn new(bb: IAabb) -> Self {
        // Octrees are cubes with sides of power of two length, so make sure we have a cube that can store the requested space.
        let bb = bb.next_pow2();
        Self {
            bb,
            nodes: vec![Node::from_aabb(bb)], // always will be branches, but this handles an edge case of extents being zero
        }
    }

    pub fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        let mut octree = Self::new(bb);
        bb.iter().for_each(|pos| {
            assert!(
                octree.set(pos, generator.lookup(pos)),
                "voxel was out of bounds"
            )
        });
        octree
    }

    /// Sets a new voxel (returns true if none).
    pub fn set(&mut self, pos: IVec3, voxel: Option<Voxel>) -> bool {
        match voxel {
            Some(voxel) => self.insert(pos, voxel),
            None => true,
        }
    }

    /// Returns the number of voxels in the scene.
    pub fn len(&self) -> usize {
        self.nodes[0].len(&self.nodes)
    }

    /// Inserts a new voxel or returns false if out of bounds.
    pub fn insert(&mut self, pos: IVec3, voxel: Voxel) -> bool {
        let mut curr_idx = 0;
        let mut bb = self.bb;
        // find a leaf node for the voxel
        loop {
            // get an index for the current aabb and check bounds (should return some for all nodes under the root)
            let Some(idx) = bb.index_of(pos) else {
                return false;
            };

            let new_idx = self.nodes.len();

            match &mut self.nodes[curr_idx] {
                Node::Branch(branches) => {
                    bb = bb.octant(idx);
                    curr_idx = match branches[idx] {
                        Some(i) => i,
                        None => {
                            // create a new node if one doesn't already exist
                            branches[idx] = Some(new_idx);
                            self.nodes.push(Node::from_aabb(bb));
                            new_idx
                        }
                    };
                }
                Node::Leaf(leaves) => {
                    leaves[idx] = Some(voxel);
                    return true;
                }
            }
        }
    }

    pub fn get(&self, pos: IVec3) -> Option<Voxel> {
        let mut curr_idx = 0;
        let mut bb = self.bb;
        loop {
            let idx = bb.index_of(pos)?;
            let octant_bb = bb.octant(idx);

            match &self.nodes[curr_idx] {
                Node::Branch(branches) => {
                    bb = octant_bb;
                    curr_idx = branches[idx]?;
                }
                Node::Leaf(leaves) => {
                    return leaves[idx];
                }
            }
        }
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        // check if ray is in branch aabb
        let range = self.bb.intersection(ray, 0.01..f32::INFINITY)?;

        let start_ray = Ray::new(ray.origin + range.start * ray.dir, ray.dir);

        self.nodes[0].trace(&self.nodes, self.bb, start_ray)
    }
}

#[derive(Debug)]
enum Node {
    Branch([Option<usize>; 8]),
    Leaf([Option<Voxel>; 8]),
}

impl Node {
    /// Creates a new node based on the size of the aabb.
    pub fn from_aabb(bb: IAabb) -> Self {
        if bb.is_unit() {
            Self::Leaf(Default::default())
        } else {
            Self::Branch(Default::default())
        }
    }

    /// Returns the number of voxels for this node.
    pub fn len(&self, nodes: &[Node]) -> usize {
        let mut count = 0;
        match self {
            Node::Branch(branches) => {
                for z in 0..2 {
                    for y in 0..2 {
                        for x in 0..2 {
                            let local_idx = x | y << 1 | z << 2;
                            let Some(next_idx) = branches[local_idx] else {
                                continue;
                            };
                            count += nodes[next_idx].len(nodes);
                        }
                    }
                }
            }
            Node::Leaf(leaves) => {
                for z in 0..2 {
                    for y in 0..2 {
                        for x in 0..2 {
                            let local_idx = x | y << 1 | z << 2;
                            if leaves[local_idx].is_some() {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    /// Trace a ray inside of this node.
    pub fn trace(&self, nodes: &[Node], bb: IAabb, ray: Ray) -> Option<Voxel> {
        let mut idx = ray.origin.cmpgt(bb.origin.as_vec3a()).bitmask() as usize;
        let mut dirs = ordered_dirs(bb, ray);

        match self {
            Node::Branch(branches) => loop {
                let Some(next_node) = branches[idx] else {
                    idx ^= 1 << dirs.next()?;
                    continue;
                };

                let next_bb = bb.octant(idx);

                let Some(range) = next_bb.intersection(ray, 0.01..f32::INFINITY) else {
                    idx ^= 1 << dirs.next()?;
                    continue;
                };

                let start_ray = Ray::new(ray.origin + range.start * ray.dir, ray.dir);

                let Some(voxel) = nodes[next_node].trace(nodes, next_bb, start_ray) else {
                    idx ^= 1 << dirs.next()?;
                    continue;
                };

                return Some(voxel);
            },
            Node::Leaf(leaves) => loop {
                let Some(voxel) = leaves[idx] else {
                    idx ^= 1 << dirs.next()?;
                    continue;
                };
                return Some(voxel);
            },
        }
    }
}

/// Sorts and filters the directions to toggle.
fn ordered_dirs(bb: IAabb, ray: Ray) -> impl Iterator<Item = usize> {
    let tests = bb.plane_intersections(ray);
    let mut axes = [(0, tests[0]), (1, tests[1]), (2, tests[2])];
    axes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    axes.into_iter()
        .filter(move |(_, s)| s.is_some())
        .map(|(i, _)| i)
}

#[cfg(test)]

mod tests {
    use glam::U8Vec3;

    use super::*;

    #[test]
    fn test_octree_insert_and_get_one() {
        let mut octree = Octree::new(IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let inserted = octree.insert(IVec3::ONE, Voxel { color: U8Vec3::ONE });
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(got, Some(Voxel { color: U8Vec3::ONE }));
        }

        {
            let inserted = octree.insert(
                IVec3::ZERO,
                Voxel {
                    color: 2 * U8Vec3::ONE,
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let inserted = octree.insert(
                2 * IVec3::NEG_ONE,
                Voxel {
                    color: 3 * U8Vec3::ONE,
                },
            );
            assert!(!inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ZERO);
            assert_eq!(
                got,
                Some(Voxel {
                    color: 2 * U8Vec3::ONE
                })
            );
        }

        {
            let got = octree.get(IVec3::NEG_ONE);
            assert_eq!(got, None);
        }

        {
            let got = octree.get(2 * IVec3::NEG_ONE);
            assert_eq!(got, None);
        }

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(got, Some(Voxel { color: U8Vec3::ONE }));
        }

        {
            let inserted = octree.insert(
                IVec3::ONE,
                Voxel {
                    color: 4 * U8Vec3::ONE,
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(
                got,
                Some(Voxel {
                    color: 4 * U8Vec3::ONE
                })
            );
        }

        {
            let inserted = octree.insert(
                IVec3::new(1, 0, 1),
                Voxel {
                    color: U8Vec3::new(0, 1, 0),
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::new(1, 0, 1));
            assert_eq!(
                got,
                Some(Voxel {
                    color: U8Vec3::new(0, 1, 0)
                })
            );
        }
    }

    #[test]
    fn test_octree_insert_and_get_two() {
        let mut octree = Octree::new(IAabb::new(IVec3::ZERO, 2 * IVec3::ONE));

        println!("{:?}", octree.nodes);

        {
            let inserted = octree.insert(IVec3::ONE, Voxel { color: U8Vec3::ONE });
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(got, Some(Voxel { color: U8Vec3::ONE }));
        }

        {
            let inserted = octree.insert(
                IVec3::ZERO,
                Voxel {
                    color: 2 * U8Vec3::ONE,
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let inserted = octree.insert(
                IVec3::NEG_ONE,
                Voxel {
                    color: 3 * U8Vec3::ONE,
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ZERO);
            assert_eq!(
                got,
                Some(Voxel {
                    color: 2 * U8Vec3::ONE
                })
            );
        }

        {
            let got = octree.get(IVec3::NEG_ONE);
            assert_eq!(
                got,
                Some(Voxel {
                    color: 3 * U8Vec3::ONE
                })
            );
        }

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(got, Some(Voxel { color: U8Vec3::ONE }));
        }

        {
            let inserted = octree.insert(
                IVec3::ONE,
                Voxel {
                    color: 4 * U8Vec3::ONE,
                },
            );
            assert!(inserted);
        }

        println!("{:?}", octree.nodes);

        {
            let got = octree.get(IVec3::ONE);
            assert_eq!(
                got,
                Some(Voxel {
                    color: 4 * U8Vec3::ONE
                })
            );
        }
    }
}

#[cfg(test)]
mod trace_tests {
    use glam::{IVec3, U8Vec3, Vec3A};

    use crate::{
        ray_tracer::types::{IAabb, Ray},
        voxel::Voxel,
    };

    use super::Octree;

    #[test]
    fn get_voxel_full() {
        let mut octree = Octree::new(IAabb::new(IVec3::ZERO, IVec3::ONE));
        octree.insert(IVec3::new(0, 0, 0), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(1, 0, 0), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(0, 1, 0), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(1, 1, 0), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(0, 0, 1), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(1, 0, 1), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(0, 1, 1), Voxel { color: U8Vec3::ONE });
        octree.insert(IVec3::new(1, 1, 1), Voxel { color: U8Vec3::ONE });

        {
            let ray = Ray::new(Vec3A::new(0.0, -5.0, 0.0), Vec3A::Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }
    }

    #[test]
    fn get_voxel_one() {
        let mut octree = Octree::new(IAabb::new(IVec3::ZERO, IVec3::ONE));
        octree.insert(IVec3::new(0, 0, 0), Voxel { color: U8Vec3::ONE });

        {
            let ray = Ray::new(Vec3A::new(-0.5, -5.0, -0.5), Vec3A::Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(voxel, Voxel { color: U8Vec3::ONE });
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, -5.0, 0.5), Vec3A::Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray);
            assert_eq!(voxel, None);
        }
    }

    #[test]
    fn get_voxel_none() {
        let octree = Octree::new(IAabb::new(IVec3::ZERO, IVec3::ONE));

        {
            let ray = Ray::new(Vec3A::new(0.5, -5.0, 0.5), Vec3A::Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray);
            assert_eq!(voxel, None);
        }
    }

    #[test]
    fn get_voxel_dirs() {
        let mut octree = Octree::new(IAabb::new(IVec3::ZERO, 2 * IVec3::ONE));

        macro_rules! add {
            ($x:expr, $y:expr, $z:expr) => {{
                let color = U8Vec3::new($x, $y, $z);
                octree.insert(color.as_ivec3(), Voxel { color });
            }};
        }

        add!(0, 0, 0);
        add!(1, 0, 0);
        add!(0, 1, 0);
        add!(1, 1, 0);
        add!(0, 0, 1);
        add!(1, 0, 1);
        add!(0, 1, 1);
        add!(1, 1, 1);

        {
            let ray = Ray::new(Vec3A::new(-0.5, -5.0, -0.5), Vec3A::Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 0, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-5.0, -0.5, 0.5), Vec3A::X);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 0, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-0.5, 5.0, -0.5), Vec3A::NEG_Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 1, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(-0.5, 5.0, 0.5), Vec3A::NEG_Y);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(0, 1, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(5.0, -0.5, -0.5), Vec3A::NEG_X);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 0, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(5.0, -0.5, 0.5), Vec3A::NEG_X);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 0, 1)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, 0.5, -5.0), Vec3A::Z);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 1, 0)
                }
            );
        }

        {
            let ray = Ray::new(Vec3A::new(0.5, 0.5, 5.0), Vec3A::NEG_Z);
            assert!(octree.bb.intersection(ray, 0.01..f32::INFINITY).is_some());
            let voxel = octree.trace(ray).expect("voxel not found");
            assert_eq!(
                voxel,
                Voxel {
                    color: U8Vec3::new(1, 1, 1)
                }
            );
        }
    }
}
