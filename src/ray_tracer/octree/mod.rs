use glam::{IVec3, U8Vec3};

use crate::{
    voxel::{Voxel, VoxelGenerator}
};

use super::{
    types::{IAabb, Ray},
    Scene,
};

pub struct SparseStorage {
    octree: Octree,
}

impl Scene for SparseStorage {
    fn from_voxels(generator: &VoxelGenerator, bb: IAabb) -> Self {
        todo!()
    }

    fn trace(&self, ray: Ray) -> Option<Voxel> {
        todo!()
    }
}


pub struct Octree 
{
    pub nodes: Pool<Node>,
    pub root: NodeId, 
}

pub struct Pool<T> 
{
    elements: Vec<T>,
}

impl <T> Pool<T> 
{
    pub fn new() -> Self 
    {
        Self { elements: Vec::new() }
    }

    pub fn insert(&mut self, item: T) -> NodeId
    {
        self.elements.push(item);
        NodeId(self.elements.len() as u32 - 1)
    }

    pub fn get(&self, id: NodeId) -> Option<&T> 
    {
        self.elements.get(id.0 as usize)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T>
    {
        self.elements.get_mut(id.0 as usize)
    }

}
#[derive(Clone, Copy)]
pub struct NodeId(pub u32);

pub struct Node {
    pub aabb: IAabb,
    pub ntype: NodeType,
    pub parent: Option<NodeId>,
}

impl Default for Node{
    fn default() -> Self {
        Node {
            aabb: IAabb::new(IVec3::ZERO, IVec3::ZERO),
            ntype: Default::default(),
            parent: Default::default(),
        }
    }
}

impl Node {
    pub(crate) fn from_aabb(aabb: IAabb, parent:Option<NodeId>) -> Self
    {
        Self 
        {
            aabb,
            parent,
            ..Default::default() // makes ntype the default (empty)
        }
    }
}

#[derive(Default, Clone)]
pub enum NodeType {
    #[default] // makes default empty
    Empty,
    Leaf(Voxel),
    Branch(Branch),
}

pub struct Branch 
{
    pub children: [NodeId; 8],
}

impl Branch 
{
    pub(crate) fn new(children: [NodeId; 8]) -> Self
    {
        Branch { children }
    }
    pub fn x0_y0_z0(&self) -> NodeId
    {
        self.children[0]
    }
    pub fn x1_y0_z0(&self) -> NodeId
    {
        self.children[1]
    } 
    pub fn x0_y1_z0(&self) -> NodeId
    {
        self.children[2]
    } 
    pub fn x0_y0_z1(&self) -> NodeId
    {
        self.children[3]
    } 
    pub fn x1_y1_z0(&self) -> NodeId
    {
        self.children[4]
    } 
    pub fn x1_y0_z1(&self) -> NodeId
    {
        self.children[5]
    } 
    pub fn x0_y1_z1(&self) -> NodeId
    {
        self.children[6]
    }
    pub fn x1_y1_z1(&self) -> NodeId
    {
        self.children[7]
    }
    pub fn center(&self, nodes: &Pool<Node>) -> IVec3 
    {
        let mut sum = IVec3::ZERO;
        for &child_id in &self.children 
        {
            if let Some(child) = nodes.get(child_id)
            {
                sum += child.aabb.origin;
            }
        }
        sum / 8
    }
}

impl Octree 
{
    pub fn insert(&mut self, voxel: Voxel) 
    {
        let mut node_id = self.root;
        while let Some(node) = self.nodes.get_mut(node_id)
        {
            match node.ntype {
                NodeType::Empty => 
                {
                    node.ntype = NodeType::Leaf(voxel);
                    return;
                }
                NodeType::Leaf(existing_voxel) =>
                {
                    if existing_voxel == voxel
                    {
                        return;
                    }
                    self.split(node_id, existing_voxel, voxel);
                    return;
                }
                NodeType::Branch(ref mut branch) => 
                {
                    let child_index = self.get_child_index(&node.aabb, voxel.pos);
                    node_id = branch.children[child_index];
                }
            }
        }
    }    

    fn split(&mut self, node_id: NodeId, old_voxel: Voxel, new_voxel: Voxel)
    {
        if let Some(leaf) = self.nodes.get_mut(node_id)
        {
            let mut branch = Branch{children: [NodeId(0); 8]};
            for i in 0..8
            {
                let new_aabb = leaf.aabb.split(i);
                branch.children[i] = self.nodes.insert(Node::from_aabb(new_aabb, Some(node_id)));
            }
            let child_index_old = self.get_child_index(&leaf.aabb, old_voxel.pos);
            let child_index_new = self.get_child_index(&leaf.aabb, new_voxel.pos);
            
            if let Some(child) = self.nodes.get_mut(branch.children[child_index_old])
            {
                child.ntype = NodeType::Leaf(new_voxel);
            }

            if let Some(child) = self.nodes.get_mut(branch.children[child_index_new]) {
                child.ntype = NodeType::Leaf(new_voxel);
            }

            leaf.ntype = NodeType::Branch(branch);

        }
    }

    fn get_child_index(&self, aabb: &IAabb, pos: IVec3) -> usize
    {
        let center = (aabb.min() + aabb.max()) / 2;
        let mut index = 0;
        if pos.x >= center.x { index |= 1; }
        if pos.y >= center.y { index |= 2; }
        if pos.z >= center.z { index |= 4; }
        index
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_octree_insert() 
    {
        let mut octree = Octree {
            nodes: Pool::new(),
            root: NodeId(0),
        };
        let voxel = Voxel {pos: IVec3::new(0,0,0), color: U8Vec3::new(255,0,0)};
    
        octree.insert(voxel);

        assert!(matches!(octree.nodes.get(octree.root).unwrap().ntype, NodeType::Leaf(_)))
    }
}