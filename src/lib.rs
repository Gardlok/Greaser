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
    Future,                                 //
    FutureExt,                              //
    StreamExt,                              //
};
//
use std::collections::HashMap;
use std::pin::Pin;

use thiserror::Error;

mod attri;
mod craft;
mod data;
mod edge;
mod index;
mod node;
mod param;
mod test;
use attri::*;
use craft::{DataCraft as DC, EdgeCraft as LC, NodeCraft as NC};
use data::*;
use edge::*;
use index::*;
use node::*;
use param::*;
/////////////////////////////////////////////
// Types

pub type Func<T> = Pin<Box<dyn Future<Output = Result<Node, T>>>>;
pub type Sharedcast = (BroadCastTx<()>, BroadCastRx<()>);
/////////////////////////////////////////////
// Primary logic and work flow
pub struct Matrices {
    // Map of Node ID to a copy of individual node Matrisync
    // objects. Any node operating in any scope must have a
    // copy of their matrisync in this map
    msyncs: HashMap<Node, Vec<Matrisync>>,
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

pub struct Matricet;

pub struct Matrisync {
    pub id: Node,
    j_handle: Option<JoinHandle<Node>>,
    a_handle: Option<AbortHandle>,
    t_handle: Option<RtHandle>,
    matricet: Matricet,
}

impl Matrisync {
    pub async fn new() -> Matrisync {
        Matrisync {
            id: Node::new(),
            j_handle: None,
            a_handle: None,
            t_handle: None,
            matricet: Matricet,
        }
    }

    pub fn sync(&mut self, other: Matrisync) {
        if self.id.0 != other.id.0 {
            return;
        }
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
