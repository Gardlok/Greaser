#![feature(associated_type_bounds)]
#[allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    unused_imports,
    unsafe_op_in_unsafe_fn
)]
//
use tokio::runtime::{Builder, Handle as RtHandle, Runtime};
use tokio::sync::broadcast::{
    channel as bc_channel,   //
    Receiver as BroadCastRx, //
    Sender as BroadCastTx,   //
};
use tokio::sync::watch::{channel as w_channel, Receiver as WatchRx, Sender as WatchTx};
use tokio::sync::{Barrier, OnceCell};
use tokio::task::{spawn, AbortHandle, JoinHandle, JoinSet};
use tokio::task_local;
use tokio::time::{Duration, Instant};
//
use futures::{
    stream::FuturesOrdered as ord_futs,     //
    stream::FuturesUnordered as unord_futs, //
    task::FutureObj,                        //
    Future,                                 //
    FutureExt,                              //
    StreamExt,                              //
};
use std::borrow::BorrowMut;
//
use std::collections::HashMap;
use std::iter::OnceWith;
use std::marker::PhantomData;
use std::pin::Pin;

use thiserror::Error;

mod crafting;
mod data;
mod edge;
mod node;
mod parameter;
mod test;
use builder::parameter::Param;
use crafting::{DataCraft as DC, EdgeCraft as LC, NodeCraft as NC};
/////////////////////////////////////////////
// Types

type Func<T> = Pin<Box<dyn Future<Output = Result<Node, ()>>>>;
type Sharedcast = (BroadCastTx<()>, BroadCastRx<()>);
impl trythat for Node {}
/////////////////////////////////////////////
// Primary logic and work flow
pub struct Matrices {
    // Map of Node ID to a copy of individual node Matrisync
    // objects. Any node operating in any scope must have a
    // copy of their matrisync in this map
    msyncs: HashMap<Node<(), ()>, Vec<Matrisync<(), ()>>>,
    // Store the runtimes and tasks in their async function form
    //mfuncs: [Func],
    //
    // Temp experiment /////////////////////////////////////////
    // msyncs: Pin<Box<HashMap<Nid, Vec<Matrisync<NodeType>>>>>,
    mfuncs: Pin<Box<[Func<()>]>>, //
    ////////////////////////////////////////////////////////////
    // Create/store a MPMC channel to be distrobuted among the
    // nodes. For systematic purpose, this will be the _shared_
    // copy while a seperate copy will be used for the Matrices
    // deticated root channel.
    sharedcast: LC::SharedCast<LC::Watch<()>>,
}

impl Matrices {
    // pub fn add(&mut self, msync: Matrisync<NC::NodeType>) -> Result<(), ()> {
    //     let x = self
    //         .msyncs
    //         .entry(msync.id.get().clone())
    //         //.and_modify(|v| v.push(msync))
    //         .or_insert(vec![msync]);

    //     Ok(())
    // }
    // pub fn new_node(self) -> Node {
    //     Node(0)
    // }
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

pub struct Matrisync<S, T> {
    id: Node<S, T>,
    state: Param<NC::State>,
    idle: Param<bool>,
    j_handle: Option<JoinHandle<Node<(), T>>>,
    a_handle: Option<AbortHandle>,
    t_handle: Option<RtHandle>,
    edges: Param<HashMap<DC::NodeInfo, DC::EdgeInfo>>,
    _attr: PhantomData<Node<(), T>>,
}

impl<S, T> Matrisync<S, T> {
    fn new() -> Matrisync<S, T> {
        Matrisync {
            id: Node(OnceCell::new(), PhantomData),
            state: Param::NotSet(NC::State::Setup),
            idle: Param::NotSet(false),
            j_handle: None,
            a_handle: None,
            t_handle: None,
            edges: Param::NotSet(HashMap::new()),
            _attr: PhantomData,
        }
    }

    fn sync(&mut self, other: Matrisync<(), T>) {
        if self.id.0 != other.id.0 {
            return;
        }
        self.state = other.state;
        self.idle = other.idle;
        self.j_handle = other.j_handle;
        self.a_handle = other.a_handle;
        self.t_handle = other.t_handle;
    }
}

//////////////////////////////////////////////////
// Go time!
/////////////////
pub(crate) mod Engine {
    use super::*;
    pub fn start() -> () {
        ()
    }
    pub fn get_ready() -> () {}
    pub fn get_set() -> () {
        ()
    }
    macro_rules! GO {
        () => {};
    }
}
