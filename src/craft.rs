//

// CommonSpace
use futures::Future;
use std::pin::Pin;
//
// Node, Edge, and Data Crafting
// pub use crate::{index::*, param::*};
pub use {DataCraft::*, EdgeCraft::*, NodeCraft::*};
pub enum Element {
    //////// Relative ID's
    Node, // Noid -  Node ID
    Edge, // Egid -  Edge ID
    Data, // Scid - Scope ID
}
/////////////////////////////////////////////////////////////////////////

pub mod DataCraft {
    pub use hashbrown::{HashMap, HashSet};

    // Parsing data:
    //   1) Edge Layer (Concerned with Sender, reciever node IDs)
    //   2) Data Layer (Concerned with Class and Priority of Content)
    //   3) Exec Layer (Concerned with Ownership, Allocation, Handling)
    //   4) Rins Layer (Concerned with Reporting, Cleanup, Dropping)
    // pub trait Data {}
    /////////////////////////////////////////////////////////////////////
    //
    //
    // pub trait LayeredParsing {}
    // pub trait EdgeLayer<TX, RX> {}
    // pub trait DataLayer<C, P> {}
    // pub trait ExecLayer {}
    // pub trait RinsLayer {}
    pub struct EdgeLayer {
        pub source: TX,
        pub dest: RX,
    }
    pub trait DataLayer<C, P> {}
    pub trait ExecLayer {}
    pub trait RinsLayer {}

    pub struct Data {
        pub edge_layer: EdgeLayer,
    }

    /////////////////////////////////////////////////////////////////////
    // Scope Designation
    //
    #[derive(PartialEq, Eq, Hash)]
    pub enum NodeInfo {
        Noid(u8), // Node ID
        Rtid(u8), // Runtime ID
        Ntid(u8), // Type ID
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum EdgeInfo {
        Liid(u8), // Edge ID
        Scid(u8), // Scope ID
        Ltid(u8), // Type ID
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum DataInfo {
        Daid(u8), // Data ID
        Foid(u8), // Format ID
        Dtid(u8), // Type ID
    }

    /////////////////////////////////////////////////////////////////////
    // Dynamic Dispatching
    //
    use std::any::{Any, TypeId};
    use std::hash::{Hash, Hasher};

    pub trait DynEq: Any {
        fn dyn_eq(&self, other: &dyn DynEq) -> bool;
        fn as_any(&self) -> &dyn Any;
    }
    pub trait DynHash: DynEq {
        fn dyn_hash(&self, hasher: &mut dyn Hasher);
        fn as_dyn_eq(&self) -> &dyn DynEq;
    }
}

pub mod NodeCraft {
    use tokio::sync::OnceCell;

    pub trait Node {}

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Noid(pub OnceCell<u8>, pub OnceCell<u8>);

    //
}
///////////////////////////////////////////////////////////////////////////////
pub mod EdgeCraft {
    /*
        Edges are the relationships, connecting points, and behaivors that have
        some effect or meaning transferred from one node to another. These
        definitions will structure the shared resources.
        Primary Types:
            MatriSet - Base level ops messaging through broadcast and unique addresses. Includes a
                        Matridex which catalogs nodes connected to the MatriSet channel.
            EdgeSet - Varying Collection of Edges, these will extend behaivors beyond the MatriSet
                      capabilities
    */
    use crate::DataCraft::{DynEq, DynHash, HashSet};
    use crate::NodeCraft::Noid;
    //
    pub use tokio::sync::{
        broadcast::{channel as b_chan, Receiver as BCrx, Sender as BCtx},
        mpsc::{channel as mpsc_chan, Receiver as MPSCrx, Sender as MPSCtx},
        oneshot::{channel as os_chan, Receiver as OSrx, Sender as OStx},
        watch::{channel as w_chan, Receiver as Wrx, Sender as Wtx},
        Barrier, Mutex, OnceCell, RwLock,
    };
    pub use tokio::{
        runtime::Handle,
        task::{AbortHandle, JoinHandle},
    };

    pub trait Edge {}
    //
    //////////
    // EdgeSet
    // Common Edge types are the tools/tasks a node will utilize.
    pub trait EdgeHash: DynHash + DynEq + Edge {}
    pub struct EdgeSet(HashSet<Box<dyn EdgeHash>>);
    //
    #[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EdgeType {
        Buffr, // BCrx|MPSCrx -> RwLock buffer -> ?
        Linkr, // Source: MPSCtx & MPSCrx <--> Destination: MPSCtx & MPSCrx
        Sortr, // Blocking Channel/Content sorting task
        Findr, // ?
        Waitr, // Queue for readiness within a group of tasks
    }

    // Error Handling for Edges TODO: More...
    #[derive(thiserror::Error, Debug)]
    pub enum EdgeError<T> {
        #[error("Error Sending Data")]
        SendFailMPSC(#[from] tokio::sync::mpsc::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFailOS(#[from] tokio::sync::oneshot::error::RecvError),
        #[error("Error Sending Data")]
        SendFailBC(#[from] tokio::sync::broadcast::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFailBC(#[from] tokio::sync::broadcast::error::RecvError),
    }
}
///////////////////////////////////////////////////////////////////////////////
pub mod Crafting {
    use super::*;
    use hashbrown::HashMap;

    pub struct CraftWrap<Item, ParamSet> {
        item: Option<Item>,
        params: HashMap<ParamSet, Option<ParamSet>>,
    }
    impl<Item, ParamSet> CraftWrap<Item, ParamSet> {
        pub fn new(item: Item) -> CraftWrap<Item, ParamSet> {
            CraftWrap {
                item: None,
                params: HashMap::new(),
            }
        }
    }
    pub type FutFunc<T> = Pin<Box<dyn Future<Output = Result<T, ()>>>>;

    pub trait Craftable
    where
        Self: Sized,
    {
        type Item;
        type ParamSet;
        fn init(/*  Item  */) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn __def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn __sco<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn conf<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn lock<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
        fn free<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    }
}
//mod crafting;

// fn build_node(self) -> Result<Runtime, ()> {
//     Builder::new_current_thread()
//         .thread_name(self.name)
//         .on_thread_start(|| ()) // TODO!
//         .on_thread_stop(|| ()) // TODO!
//         .on_thread_park(|| ()) // TODO!
//         .on_thread_unpark(|| ()) // TODO!
//         .enable_time()
//         .enable_io()
//         .start_paused(true)
//         .build()
//         .map_err(|_| ()) // TODO!
//         .map(|rt| rt) // TODO!
// }
