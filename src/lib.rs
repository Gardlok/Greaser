// #![feature(maybe_uninit_ref)]
#![feature(async_fn_in_trait)]
#![feature(box_patterns)]
#![feature(associated_type_bounds)]
#![feature(trivial_bounds)]
#![feature(stdsimd)]

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
//
use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};
//
use hashbrown::HashMap;
use std::pin::Pin;
//

pub mod biblio; // Measuring, observations, and Smart pointers
pub mod craft; // Base Declerations
pub mod data; // Data portion of D.E.N's logistics model set
pub mod edge; // Edge portion of D.E.N's logistics model set
pub mod node; // Node portion of D.E.N's logistics model set
pub mod param; // Like Option, but returns an ITEM either way
               //
               //
use biblio::NestIndex;

#[cfg(test)]
mod test;
//
pub const MAG_NUM: u8 = 6;
//
pub type FutFunc<T> = Pin<Box<dyn Future<Output = Result<T, ()>>>>;
/////////////////////////////////////////////
// Primary logic and work flow
pub struct Matrices {
    // Map of Node info to a copy of individual node MatriSet
    // objects. Any node operating in any scope must have a
    // copy of their Sigma in this map
    sigma: HashMap<Node, Sigma>,
    ////////////////////////////////////////////////////////////
    //
    //
    mfunc: HashMap<Node, Pin<Box<[FutFunc<()>]>>>,
    ////////////////////////////////////////////////////////////
    // Store the runtime and tasks in their handle form to be
    // managed after their instance has been created
    rhand: HashMap<Node, Runtime>,
    thand: HashMap<Node, JoinHandle<()>>,
    ////////////////////////////////////////////////////////////
    // Create/store a Broadcast Stream to be distrobuted among the
    // nodes.
    matri: MatriStream,
    ////////////////////////////////////////////////////////////
    //
    // Nested index to manage Node indices
    noded: NestIndex::NestIndex,
}

impl Matrices {
    pub async fn new() -> Matrices {
        Matrices {
            sigma: HashMap::new(),
            mfunc: HashMap::new(),
            rhand: HashMap::new(),
            thand: HashMap::new(),
            matri: MatriStream::new().await,
            noded: NestIndex::NestIndex::new(),
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
            sigma.node = Node::from(rtid, noid);
        } else {
            let (rtid, noid) = self.noded.next();
            sigma.node = Node::from(rtid, noid);
        }

        //
        sigma.matristream = self.matri.clone();

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
    node: Node,
    matristream: MatriStream,
}
impl Sigma {
    pub async fn new() -> Sigma {
        Sigma {
            node: Node::new(),
            matristream: MatriStream::new().await,
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
    // macro_rules! GO {
    //     () => {};
    // }
}
