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
use std::pin::Pin;
//

pub mod biblio; // Measuring, observations, and Smart pointers
pub mod craft; // Base Declerations
pub mod data; // Data portion of D.E.N's logistics model set
pub mod edge; // Edge portion of D.E.N's logistics model set
pub mod node; // Node portion of D.E.N's logistics model set
              //
              //
use biblio::NestIndex;

#[cfg(test)]
mod test;
//
pub const MAG_NUM: u8 = 6;

use tokio::sync::broadcast;

pub enum FactoryCommand {
    CreateNode {
        id: NodeId,
        attributes: NodeAttributes,
    },
    UpdateNode {
        id: NodeId,
        attributes: NodeAttributes,
    },
    DeleteNode(NodeId),
    CreateEdge {
        from: NodeId,
        to: NodeId,
        attributes: EdgeAttributes,
    },
    UpdateEdge {
        from: NodeId,
        to: NodeId,
        attributes: EdgeAttributes,
    },
    DeleteEdge {
        from: NodeId,
        to: NodeId,
    },
    NodeOp {
        node_id: NodeId,
        operation: NodeOps,
    },
    BeginScope(Scope),
    EndScope(Scope),
    Sleep,
    Wake,
    Query(QueryType),
    // Add more commands as needed...
}

    // Existing methods...TODO

    pub async fn listen_to_broadcast(&mut self) {
        while let Ok(command) = self.receiver.recv().await {
            match command {
                FactoryCommand::CreateClient { interest, client } => {
                    self.clients.insert(interest.clone(), client);
                    let handle = self.run_client(&interest).await;
                    self.running_clients.insert(interest, handle);
                }
                FactoryCommand::RunClient { interest } => {
                    if let Some(handle) = self.run_client(&interest).await {
                        self.running_clients.insert(interest, handle);
                    }
                }
                FactoryCommand::StopClient { interest } => {
                    if let Some(handle) = self.running_clients.remove(&interest) {
                        handle.abort();
                    }
                }
                FactoryCommand::CreateTemplate { interest, template } => {
                    self.templates.insert(interest, template);
                }
            }
        }
    }

    async fn run_client(&mut self, interest: &Array2<u8>) -> Option<tokio::task::JoinHandle<()>> {
        if let Some(client) = self.clients.get(interest) {
            // Some(tokio::spawn(async move {

            //     // Run client...TODO
            // }))
        } else {
            None
        }
    }
}

// let mask: u32 = /* some value that represents the bits you're interested in */;
// let handlers = HashMap::new();
// populate handlers with functions or data for each possible value of the mask
// for message in message_stream {
//     let node = message.get_node();
//     let masked_key = node.get_first_important_part() & mask;
//     if let Some(handler) = handlers.get(&masked_key) {
//             // process message using handler
//         }
// }

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
    // Channels for incoming commands and outgoing responses.
    command_rx: Receiver<FactoryCommand>,
    response_tx: Sender<FactoryResponse>,
    ////////////////////////////////////////////////////////////
    // Store the runtime and tasks in their handle form to be
    // managed after their instance has been created
    rhand: HashMap<Node, Runtime>,
    thand: HashMap<Node, JoinHandle<()>>,
    ////////////////////////////////////////////////////////////
    // Maintains a list of all active edges.
    edges: HashMap<(NodeId, NodeId), Edge>,
    // Create/store a Broadcast Stream to be distrobuted among the
    // nodes.
    matri: MatriSync,
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
            matri: MatriSync::new().await,
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
            sigma.node = Node::from(rtid);
        } else {
            let (rtid, noid) = self.noded.next();
            sigma.node = Node::from(rtid);
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
