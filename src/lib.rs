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
mod thashmap;
use attri::*;
use craft::*;
use data::*;
use edge::*;
use index::*;
use node::*;
use param::*;
use thashmap::*;
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
    sigmas: HashMap<u8, Sigma>,
    // Store the runtimes and tasks in their async function form
    //mfuncs: [Func],
    //
    // Temp experiment /////////////////////////////////////////
    // msyncs: Pin<Box<HashMap<Nid, Vec<Matrisync<NodeType>>>>>,
    mfuncs: Pin<Box<[FutFunc<()>]>>, //
    ////////////////////////////////////////////////////////////
    // Create/store a MPMC channel to be distrobuted among the
    // nodes. For systematic purpose, this will be the _shared_
    // copy while a seperate copy will be used for the Matrices
    // deticated root channel.
    matrisync: Matrisync<()>,
    ////////////////////////////////////////////////////////////
    // Multi Tally Tool using types as the index.
    // TODO:  Implement more metrics
    indexer: TypeTally,
}

impl Matrices {
    pub fn add(&mut self, sigma: Sigma) -> Result<(), ()> {
        // If Matrices already has existing Node under this
        // handle we <<  [X]"will"  [ ]"will not"  >> overwrite it
        let x = self
            .sigmas
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
    handle: (), //Handles<()>,
    matrisync: Matrisync<()>,
}
impl Sigma {
    pub async fn new() -> Sigma {
        Sigma {
            node: Node::new(),
            handle: (), //                   <TODO:
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
