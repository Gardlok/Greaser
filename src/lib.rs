#![feature(maybe_uninit_ref)]
#![feature(associated_type_bounds)]
use futures::Future;
#[allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    unused_imports,
    unsafe_op_in_unsafe_fn
)]
//
use tokio::runtime::{Builder, Handle as RtHandle, Runtime};
use tokio::time::{Duration, Instant};
//
use hashbrown::HashMap;
use std::pin::Pin;
//
use thiserror::Error;

mod attri; // Working Types as keys
mod craft; // Base Declerations
mod data; // Data portion of D.E.N's logistics model set
mod edge; // Edge portion of D.E.N's logistics model set
mod index; // Measuring, observations, and Smart pointers
mod node; // Node portion of D.E.N's logistics model set
mod param; // Like Option, but returns an ITEM either way
use attri::*;
use craft::*;
use data::*;
use edge::*;
use index::*;
use node::*;
use param::*;
//
//
#[cfg(test)]
mod test;
//
//
/////////////////////////////////////////////
// Primary logic and work flow
pub struct Matrices {
    // Map of Node ID to a copy of individual node Matrisync
    // objects. Any node operating in any scope must have a
    // copy of their matrisync in this map
    sigma: HashMap<Node, Sigma>,
    // Store the runtimes and tasks in their async function form
    //mfuncs: [Func],
    //
    // Temp experiment /////////////////////////////////////////
    // msyncs: Pin<Box<HashMap<Nid, Vec<Matrisync<NodeType>>>>>,
    mfunc: Pin<Box<[FutFunc<()>]>>, //
    ////////////////////////////////////////////////////////////
    // Store the runtime and tasks in their handle form to be
    // managed after their instance has been created
    rhand: HashMap<Node, Runtime>,
    thand: HashMap<Node, JoinHandle<()>>,
    ////////////////////////////////////////////////////////////
    // Create/store a MPMC channel to be distrobuted among the
    // nodes. For systematic purpose, this will be the _shared_
    // copy while a seperate copy will be used for the Matrices
    // deticated root channel.
    matrisync: Matrisync<()>,
    ////////////////////////////////////////////////////////////
    //
    // Multi Tally Tool using types as the index.
    type_indexer: TypeTally,
}

impl Matrices {
    pub async fn add(&mut self, rtid: Option<u8>, futfunc: FutFunc<()>) -> Result<(), ()> {
        // If Matrices already has existing Node under this
        // handle we <<  [X]"will"  [ ]"will not"  >> overwrite it
        // Pending further review...subject to change

        
        
        let mut sigma = Sigma::new().await;
        sigma.node = () 
        let x = self
            .sigma
            .entry(sigma.node.0.get().unwrap().to_owned())
            .or_insert(sigma);

        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////
// Purpose
//
//
//
#[derive(Debug, Clone)]
pub struct Sigma {
    node: Node,
    matrisync: Matrisync<()>,
}
impl Sigma {
    pub async fn new() -> Sigma {
        Sigma {
            node: Node::new(),
            matrisync: Matrisync::init(),
        }
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
