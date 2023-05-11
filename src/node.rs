use crate::craft::{Crafting::*, EdgeCraft::*, NodeCraft::*};
// A node is any part of the Matrices that is not an Edge nor Data by itself. A
// node will use Edges and Data to achieve it's goal. A Node is a representation
// of any Task, Runtime, Event processing within the Matrices.

use bitvec::prelude::*;
use std::hash::{Hash, Hasher};

///////////////////////////////////////////////////////////////

use super::*;

impl Node {
    pub fn new() -> Node {
        Node {
            rid: 0u8,
            syn: Synaptics::empty(),
        }
    }
    pub fn from(rid: u8) -> Node {
        let mut n = Self::new();
        n.rid = rid;
        n
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
