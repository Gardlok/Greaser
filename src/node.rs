use crate::craft::{Crafting::*, EdgeCraft::*, NodeCraft::*};
// A node is any part of the Matrices that is not an Edge nor Data by itself. A
// node will use Edges and Data to achieve it's goal. A Node is a representation
// of any Task, Runtime, Event processing within the Matrices.

use bitvec::prelude::*;
use std::hash::{Hash, Hasher};

///////////////////////////////////////////////////////////////

use super::*;
impl NodeOps for Node {
    fn get_life_expectancy(&self) -> u8 {
        self.attributes[0] as u8 + (self.attributes[1] as u8) << 1
    }
    fn set_life_expectancy(&mut self, life_expectancy: u8) {
        self.attributes.set(0, life_expectancy & 1 != 0);
        self.attributes.set(1, life_expectancy & 2 != 0);
    }
    fn get_state(&self) -> bool {
        self.attributes[2]
    }
    fn set_state(&mut self, state: bool) {
        self.attributes.set(2, state);
    }
    fn get_node_type(&self) -> u8 {
        self.attributes[3] as u8 + (self.attributes[4] as u8) << 1
    }
    fn set_node_type(&mut self, node_type: u8) {
        self.attributes.set(3, node_type & 1 != 0);

        self.attributes.set(4, node_type & 2 != 0);
    }
    fn get_scope(&self) -> u8 {
        self.attributes[5] as u8 + (self.attributes[6] as u8) << 1 + (self.attributes[7] as u8) << 2
    }
    fn set_scope(&mut self, scope: u8) {
        self.attributes.set(5, scope & 1 != 0);
        self.attributes.set(6, scope & 2 != 0);
        self.attributes.set(7, scope & 4 != 0);
    }
}

impl Node {
    fn new() -> Node {
        Node {
            attributes: bitvec![0; 8],
        }
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rid.to_owned().hash(state);
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rid.to_owned().partial_cmp(&other.rid.to_owned())
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rid.to_owned().cmp(&other.rid.to_owned())
    }
}

// impl Craftable for Noid {
//     fn init() -> Self {
//         unimplemented!()
//     }
//     fn def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet> {
//         unimplemented!()
//     }
//     fn __def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet> {
//         unimplemented!()
//     }
//     fn __sco<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet> {
//         unimplemented!()
//     }
//     fn conf<T>(&mut self) -> Self {
//         unimplemented!()
//     }
//     fn lock<T>(&mut self) -> Self {
//         unimplemented!()
//     }
//     fn free<T>(&mut self) -> Self {
//         unimplemented!()
//     }
// }
// impl<Attr> Attributable<Node> for Node {
//     type Elem = Node;
//     fn def<Attr>(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
//     fn __def(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
// }
