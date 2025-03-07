use glam::{IVec3, U8Vec3};

/// Data associated with a single voxel.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Voxel {
    pub color: U8Vec3,
    pub pos: IVec3,
}

pub struct Node {
    pub vox: Voxel,
    pub children: [Option<Box<Node>>; 8],
}

/// Node is what stores the voxel and its children
/// 
impl Node {
    fn new(vox: Voxel) -> Self {
        Self {children: [None, None, None, None], vox}
    }
    fn insert(&mut self, index: usize, vox: Voxel)
    {
        if index < 8 {
            self.children[index] = Some(Box::new(Node::new(vox)));
        }
    }
}

/// An iterator that produces voxels.
pub struct VoxelGenerator {
    
}

impl VoxelGenerator {
    /// Create a new voxel generator.
    pub fn new() -> Self {
        todo!()
    }

    /// Lookup a voxel value at some position.
    pub fn lookup(&self, pos: IVec3) -> Option<Voxel> {
        todo!()
    }
}