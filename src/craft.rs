//

// CommonSpace
use futures::Future;
use std::marker::PhantomData;
use std::pin::Pin;
//
// Node, Edge, and Data Crafting
pub use crate::{index::*, param::*};
pub use {DataCraft::*, EdgeCraft::*, NodeCraft::*};
pub enum Element {
    Node,
    Edge,
    Data,
}
/////////////////////////////////////////////////////////////////////////

pub mod DataCraft {
    use hashbrown::HashMap;
    use std::any::{Any, TypeId};
}

pub mod NodeCraft {
    use super::*;
    use std::cmp::{Eq, PartialEq};
    use std::hash::Hash;
    use tokio::sync::OnceCell;

    pub type Node = NodeStruct<(), (), (), ()>;
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct NodeStruct<Noid, Quid, Ntid, Stid>(
        pub OnceCell<u8>,
        pub PhantomData<(Noid, Quid, Ntid, Stid)>,
    );
    pub type FutFunc<T> = Pin<Box<dyn Future<Output = Result<Node, T>>>>;
    ///
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum NodeType {
        Runtime,
        Task,
        Shadow,
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum NodeInfo {
        Noid(u8), // Node ID
        Quid(u8), // Quick ID
        Ntid(u8), // Type ID
        Stid(u8), // State ID
    }
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum State {
        Setup,
        Init,
        Run,
        Diag,
        Stop,
        End,
    }
}

pub mod EdgeCraft {
    use super::*;

    use futures::{
        stream::FuturesOrdered as ord_futs,     //
        stream::FuturesUnordered as unord_futs, //
        Future,
        FutureExt,
        StreamExt,
    };
    use tokio::runtime::Handle;
    pub use tokio::sync::{
        broadcast::{channel as b_chan, Receiver as BCrx, Sender as BCtx},
        mpsc::{channel as mpsc_chan, Receiver as MPSCrx, Sender as MPSCtx},
        oneshot::{channel as os_chan, Receiver as OSrx, Sender as OStx},
        watch::{channel as w_chan, Receiver as Wrx, Sender as Wtx},
        Barrier, OnceCell,
    };
    pub use tokio::task::{AbortHandle, JoinHandle};

    #[derive(Debug)]
    pub struct Matrisync<T>(pub BCtx<T>, pub BCrx<T>);
    pub struct Handles<T>(
        pub Param<Handle>,
        pub Option<JoinHandle<T>>,
        pub Param<AbortHandle>,
    );

    // Error Handling for Edges
    #[derive(thiserror::Error, Debug)]
    pub enum EdgeError<T> {
        #[error("Error Sending Data")]
        SendFail_mpsc(#[from] tokio::sync::mpsc::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFail_os(#[from] tokio::sync::oneshot::error::RecvError),
        #[error("Error Sending Data")]
        SendFail_bs(#[from] tokio::sync::broadcast::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFail_bs(#[from] tokio::sync::broadcast::error::RecvError),
    }

    ////////////////////////////////////////////////////////////////////
}
pub trait Craftable
where
    Self: Sized,
{
    fn init(/*Item */) -> Self;
    fn conf<T>(&mut self) -> Self;
    fn lock<T>(&mut self) -> Self;
    fn free<T>(&mut self) -> Self;
}

pub trait Tinkerable<Elem>
where
    Self: Sized,
{
    type Elem<Attr>;
    fn def<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
    fn __def<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
    fn __flip<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
    fn __incr<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
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

// #[derive(PartialEq, Eq, Hash)]
// pub enum NodeInfo {
//     Noid(u8), // Node ID
//     Quid(u8), // Quick ID
//     Ntid(u8), // Type ID
//     Stid(u8), // State ID
// }
// #[derive(PartialEq, Eq, Hash)]
// pub enum EdgeInfo {
//     Liid(u8), // Edge ID
//     Scid(u8), // Scope ID
//     Paid(u8), // Pattern ID
//     Ltid(u8), // Type ID
// }
// #[derive(PartialEq, Eq, Hash)]
// pub enum DataInfo {
//     Daid(u8), // Data ID
//     Scid(u8), // Scope ID
//     Foid(u8), // Format ID
//     Dtid(u8), // Type ID
// }
