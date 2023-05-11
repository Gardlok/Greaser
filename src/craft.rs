//

//
// Node, Edge, and Data Crafting
pub use crate::biblio::*;
pub use {DataCraft::*, EdgeCraft::*, NodeCraft::*};
/////////////////////////////////////////////////////////////////////////

pub mod DataCraft {
    use super::*;
    pub use async_bincode;
    pub use bincode::{Deserializer, Serializer};
    pub use hashbrown::{HashMap, HashSet};
    use std::any::Any;
    use std::hash::Hasher;
    // pub use tokio_bitstream_io::{BitQueue, BitReader, BitRecorder, ByteWriter};

    /////////////////////////////////////////////////////////////////////
    // data: Msg,
    // dsiz: usize,
    // dcnt: usize,
    // tsiz: usize,
    // algn: bool,
    // buff: Vec<Vec<&'static [()]>>,

    ///////////////////////
    // Dynamic Dispatching
    //

    pub trait DynEq: Any {
        fn dyn_eq(&self, other: &dyn DynEq) -> bool;
        fn as_any(&self) -> &dyn Any;
    }
    pub trait DynHash: DynEq {
        fn dyn_hash(&self, hasher: &mut dyn Hasher);
        fn as_dyn_eq(&self) -> &dyn DynEq;
    }
}

///////////////////////////////////////////////////////////////////////////////

pub mod NodeCraft {
    use super::*;

    enum Synaptic {
        Node, // 2 - Node type
        Role, // 2 - Role class
        Life, // 2 - Lifetime expectancy
        Stat, // 2 - State of overall status
        Edge, // 2 - Edge state
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Node {
        pub rid: u8,        // Runtime ID
        pub syn: Synaptics, // Synaptics
    }
}

///////////////////////////////////////////////////////////////////////////////

pub mod EdgeCraft {
    use crate::biblio::Lexicon::Lexical;
    //
    pub use polars::prelude::*;
    pub use tokio::{
        runtime::Handle,
        sync::{
            broadcast::{
                channel, error::SendError as Ber, error::SendError as Ser, Receiver as Brx,
                Sender as Btx,
            },
            Barrier, Mutex, OnceCell, RwLock,
        },
        task::{AbortHandle, JoinHandle},
    };

    //
    #[derive(Debug)]
    pub struct MatriStream(pub Btx<()>, pub Brx<()>);

    pub type Synaptics = DataFrame;
    pub trait Synaptic
    where
        Self: Lexical,
    {
        async fn cleft(pre: Self) -> Self;
        async fn post() -> Self;
    }
}
///////////////////////////////////////////////////////////////////////////////
pub mod Crafting {
    use super::*;

    // pub struct CraftWrap<Item, ParamSet> {
    //     item: Option<Item>,
    //     params: HashMap<ParamSet, Option<ParamSet>>,
    // }
    // impl<Item, ParamSet> CraftWrap<Item, ParamSet> {
    //     pub fn new(item: Item) -> CraftWrap<Item, ParamSet> {
    //         CraftWrap {
    //             item: None,
    //             params: HashMap::new(),
    //         }
    //     }
    // }

    // pub trait Craftable
    // where
    //     Self: Sized,
    // {
    //     type Item;
    //     type ParamSet;
    //     fn init(/*  Item  */) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn __def<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn __sco<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn conf<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn lock<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    //     fn free<T>(&mut self) -> CraftWrap<Self::Item, Self::ParamSet>;
    // }

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
    // pub mod CraftingTable {
    //     use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};

    //     use super::*;
    //     use hashbrown::HashMap;

    //     pub struct Table {
    //         //root// Runtimes   //Tasks
    //         root: HashMap<Node, HashMap<Node, ()>>,
    //         scopes: HashMap<Scope, Vec<Node>>,
    //     }

    //     // Real time information on the build
    //     pub struct Stats {
    //         runtimes: usize,
    //         tasks: usize,
    //         edges: usize,
    //         scopes: usize,
    //     }

    //     impl Table {
    //         //////////////////////////////////////////////
    //         // Inputs
    //         /////////
    //         // Create a new Table with at least one Runtime/Task instance. In
    //         // any use case there should be at least one of these. However, this
    //         // is not limited to just working with one. Multiple Nodes, Edges, and
    //         // Scopes should be configurable simultaneously.
    //         pub fn new(nodes: Vec<Node>) -> Table {
    //             let mut root = HashMap::new();
    //             let mut scopes = HashMap::new();
    //             for node in nodes {
    //                 root.entry(node.noid).or_insert(HashMap::new());
    //                 // scopes.entry(noid).or_insert(vec![noid]);
    //             }
    //             Table { root, scopes }
    //         }
    //         // Add a new runtime to the roster, this also addes the additional
    //         // node entry plus scope entry respectively.
    //         pub fn add_rts(self, noids: Vec<Node>) -> Table {
    //             for noid in noids {
    //                 self.root.insert(noid.0, HashMap::new());
    //                 self.scopes.entry(noid).or_insert(vec![noid]);
    //             }
    //         }
    //         // Add new tasks to the roster
    //         pub fn add_tasks(self, noids: Vec<Node>) -> Table {
    //             for noid in noids {
    //                 self.root
    //                     .entry(noid.0)
    //                     .and_modify(|rt| rt.entry(noid.1).or_insert(Vec::new()));
    //                 self.scopes.entry(noid).or_insert(vec![noid]);
    //             }
    //         }
    //         // Add Edge components to the node configuration
    //         pub fn add_edges(self, nodes: Vec<Node>, edges: Vec<()>) -> Table {
    //             for node in nodes {
    //                 self.root
    //                     .entry(node.noid)
    //                     .and_modify(|rt| rt.entry(node.rtid).or_insert(edges))
    //             }
    //         }
    //         // Add Scope designation to the nod configuration
    //         pub fn add_scopes(self, scope: Node, noids: Vec<Node>) -> Table {
    //             for noid in noids {
    //                 self.scopes.entry(scope).and_modify(|scope| {
    //                     scope
    //                         .entry(noid)
    //                         .and_modify(|group| group.append(noid))
    //                         .or_insert()
    //                 })
    //             }
    //         }
    //         //
    //         //////////////////////////////////////////////
    //         // Outputs
    //         //////////
    //         pub fn stats(self) -> Stats {
    //             let mut count_r = 0usize; // Runtime count
    //             let mut count_t = 0usize; // Task count
    //             let mut count_e = 0usize; // Edge count
    //             let mut count_s = 0usize; // Scope count
    //             self.root.drain().inspect(|(rt, noids)| {
    //                 count_r += 1;
    //                 count_t += noids.len();
    //                 noids.drain().inspect(|_, edges| count_e += edges.len());
    //             });
    //             count_s += self.scopes.len();
    //             Stats {
    //                 runtimes: count_r,
    //                 tasks: count_t,
    //                 edges: count_e,
    //                 scopes: count_s,
    //             }
    //         }
    //         pub fn runtimes(self) -> Vec<u8> {
    //             self.root.keys().collect()
    //         }
    //         pub fn nodes(self) -> Vec<Node> {
    //             self.root.values().into_iter().sort().dedup()
    //         }
    //         pub fn scopes(self) -> Vec<Scope> {
    //             self.scopes.keys().collect()
    //         }
    //     }
}
