use crate::crafting::{Craft::*, DataCraft::*, EdgeCraft::*, NodeCraft::*};



pub type Node = NodeStruct<(), ()>;

#[derive(PartialEq, Eq)]
pub struct NodeStruct<S, T>(
    // S: Scope T: Type
    OnceCell<u32>,
    PhantomData<(S, T)>,
);






impl<R> Craftable<R> for Node {
    fn init<T>() -> Result<Self, ()> {
        !unimplemented!()
    }
    fn conf<T>(self) -> Self {
        self
    }
    fn lock<T>(self) -> Self {
        self
    }
    fn free<T>(self) -> Self {
        self
    }

impl<Attr> Tinkerable<Node> for Node {
    type Elem = Node;
    fn def(&mut self, attr: &[Attr]) -> Result<Self::Item, ()> {
        self
    }
    fn __def(self, attr: &[Attr]) -> Result<Self::Item, ()> {
        self
    }
    fn __flip(self, attr: &[Attr]) -> Result<Self::Item, ()> {
        self
    }
    fn __incr(self, attr: &[Attr]) -> Result<Self::Item, ()> {
        self
    }
}
