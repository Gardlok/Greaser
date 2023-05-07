use crate::craft::{Crafting::*, NodeCraft::*};
// A node is any part of the Matrices that is not an Edge nor Data by itself. A
// node will use Edges and Data to achieve it's goal. A Node is a representation
// of any Task, Runtime, Event processing within the Matrices.

use std::hash::{Hash, Hasher};
use tokio::sync::OnceCell;

///////////////////////////////////////////////////////////////

impl Noid {
    pub fn new() -> Noid {
        Noid(OnceCell::<u8>::new(), OnceCell::<u8>::new())
    }
    pub fn from(rtid: u8, noid: u8) -> Noid {
        Noid(
            OnceCell::new_with(Some(rtid)),
            OnceCell::new_with(Some(noid)),
        )
    }
}

impl Hash for Noid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().to_owned().hash(state);
    }
}
impl PartialOrd for Noid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0.get(), self.1.get()).partial_cmp(&(other.0.get(), other.1.get()))
    }
}
impl Ord for Noid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0.get(), self.1.get()).cmp(&(other.0.get(), other.1.get()))
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
