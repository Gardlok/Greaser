//
//
//
//
//
//
// A node is any part of the Matrices that is not an Edge nor Data by itself. A
// node will use Ddges and Data to achieve it's goal. A Node is a representation
// of any Task, Runtime, Event processing within the Matrices.
use crate::craft::{Craft::*, DataCraft::*, EdgeCraft::*, NodeCraft::*};

use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::marker::PhantomData;
use tokio::sync::OnceCell;

#[derive(PartialEq, Eq, Hash)]
pub enum NodeInfo {
    Noid(u8), // Node ID
    Quid(u8), // Quick ID
    Ntid(u8), // Type ID
    Stid(u8), // State ID
}

pub type Node = NodeStruct<(), (), (), ()>;

#[derive(PartialEq, Eq)]
pub struct NodeStruct<Noid, Quid, Ntid, Stid>(
    pub OnceCell<u32>,
    pub PhantomData<(Noid, Quid, Ntid, Stid)>,
);

impl Node {
    pub fn new() -> Node {
        let id: OnceCell<u32> = OnceCell::new();
        NodeStruct::<(), (), (), ()>(id, PhantomData) as Node
    }
    pub fn from(i: u32) -> Node {
        let id: OnceCell<u32> = OnceCell::new_with(Some(i));
        NodeStruct::<(), (), (), ()>(id, PhantomData) as Node
    }
}

impl Craftable for Node {
    fn init<T>() -> Result<Self, ()> {
        // Use new id to init a new Node
        let node = Node::new().0;
        Ok(Node::from(node.get().unwrap().clone()))
    }
    // fn conf<ITEM>(&mut self) -> Self {
    //     self.0.set(ITEM.unwrap())
    // }
    // fn lock<T>(self) -> Self {
    //     //       self.0.initialized()
    // }
    // fn free<T>(self) -> Self {
    //     self
    // }
}
// impl<Attr> Attributable<Node> for Node {
//     type Elem = Node;
//     fn def<Attr>(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
//     fn __def(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
// }
