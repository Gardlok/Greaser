#![feature(maybe_uninit_ref)]
#![feature(associated_type_bounds)]
#![feature(trivial_bounds)]
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
    // Map of Node ID to a copy of individual node MatriSet
    // objects. Any node operating in any scope must have a
    // copy of their matrisync in this map
    sigma: HashMap<Noid, Sigma>,
    // Store the runtimes and tasks in their async function form
    //mfuncs: [Func],
    //
    // Temp experiment /////////////////////////////////////////
    mfunc: HashMap<Noid, Pin<Box<[FutFunc<()>]>>>,
    // mfunc: Pin<Box<[FutFunc<()>]>>, //
    ////////////////////////////////////////////////////////////
    // Store the runtime and tasks in their handle form to be
    // managed after their instance has been created
    rhand: HashMap<Noid, Runtime>,
    thand: HashMap<Noid, JoinHandle<()>>,
    ////////////////////////////////////////////////////////////
    // Create/store a MPMC channel to be distrobuted among the
    // nodes. For systematic purpose, this will be the _shared_
    // copy while a seperate copy will be used for the Matrices
    // deticated root channel.
    matri: MatriSet<()>,
    ////////////////////////////////////////////////////////////
    //
    // Multi Tally Tool using types as the index.
    // type_indexer: TypeTally,
    ////////////////////////////////////////////////////////////
    //
    // Nested index to manage Node indices
    noded: Nestindex::Nestindex,
}

impl Matrices {
    pub async fn new() -> Matrices {
        Matrices {
            sigma: HashMap::new(),
            mfunc: HashMap::new(),
            rhand: HashMap::new(),
            thand: HashMap::new(),
            matri: MatriSet::new().await,
            noded: Nestindex::Nestindex::new(),
        }
    }
    pub async fn add(&mut self, rtid: Option<u8>, futfunc: FutFunc<()>) -> Result<Sigma, ()> {
        // If Matrices already has existing Node under this
        // handle we <<  [X]"will"  [ ]"will not"  >> overwrite it
        // Pending further review...subject to change

        // TODO: Rethink/rebuild this code block
        let mut sigma = Sigma::new().await;
        if rtid.is_some() {
            let (rtid, noid) = self.noded.next_in(rtid.unwrap());
            sigma.node = Noid::from(rtid, noid);
        } else {
            let (rtid, noid) = self.noded.next();
            sigma.node = Noid::from(rtid, noid);
        }
        //
        sigma.matrisync = self.matrisync.clone();
        //
        let x = self
            .sigma
            .entry(sigma.node.clone())
            .or_insert(sigma.clone());
        //
        self.mfunc.insert(sigma.node, Pin::new(Box::new([futfunc])));

        //
        Ok(x.to_owned())
    }
}

/////////////////////////////////////////////////////////////////////////////
// Purpose
//
//
//
#[derive(Debug, Clone)]
pub struct Sigma {
    id: Noid,
    matridex: RwLock<HashSet<Noid>>,
    matriset: (BCtx<BCtype>, BCrx<BCtype>),
    edgeset: EdgeSet,
    // HashMap of IO ??
}
impl Sigma {
    pub async fn new() -> Sigma {
        Sigma {
            id: Noid::new(),
            matrisync: MatriSet::new().await,
            edgeset: EdgeSet::new(),
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
