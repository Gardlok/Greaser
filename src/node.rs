use crate::craft::{Craftable, NodeCraft::*};
// A node is any part of the Matrices that is not an Edge nor Data by itself. A
// node will use Edges and Data to achieve it's goal. A Node is a representation
// of any Task, Runtime, Event processing within the Matrices.

use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use tokio::sync::OnceCell;

///////////////////////////////////////////////////////////////
impl Node {
    pub fn new() -> Node {
        let id: OnceCell<u8> = OnceCell::new();
        NodeStruct::<(), (), (), ()>(id, PhantomData) as Node
    }
    pub fn from(i: u8) -> Node {
        let id: OnceCell<u8> = OnceCell::new_with(Some(i));
        NodeStruct::<(), (), (), ()>(id, PhantomData) as Node
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().to_owned().hash(state);
    }
}

impl Craftable for Node {
    fn init() -> Self {
        // Use new id to init a new Node
        let node = Node::new().0;
        Node::from(node.get().unwrap().clone())
    }
    fn conf<ITEM>(&mut self) -> Self {
        unimplemented!()
    }
    fn lock<T>(&mut self) -> Self {
        unimplemented!()
    }
    fn free<T>(&mut self) -> Self {
        unimplemented!()
    }
}
// impl<Attr> Attributable<Node> for Node {
//     type Elem = Node;
//     fn def<Attr>(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
//     fn __def(self, attr: Attr) -> Result<Self, ()> { self<attr>() }
// }
