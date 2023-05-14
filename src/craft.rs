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
    use std::any::Any;
    use std::hash::Hasher;

    // pub struct DatabaseAdapter {
    //     rx: broadcast::Receiver<DataFrame>,
    //     db: Arc<Mutex<SurrealDB>>, // Placeholder for your SurrealDB connection
    //     buffer: VecDeque<DataFrame>,
    //     buffer_limit: usize,
    // }

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
    // use super::*;
    use bitvec::prelude::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Node {
        pub attributes: BitVec<u8, Lsb0>,
    }

    pub trait NodeOps {
        fn get_life_expectancy(&self) -> u8;
        fn set_life_expectancy(&mut self, life_expectancy: u8);
        fn get_state(&self) -> bool;
        fn set_state(&mut self, state: bool);
        fn get_node_type(&self) -> u8;
        fn set_node_type(&mut self, node_type: u8);
        fn get_scope(&self) -> u8;
        fn set_scope(&mut self, scope: u8);
    }
}

///////////////////////////////////////////////////////////////////////////////

pub mod EdgeCraft {
    use crate::Node;
    use bytes::Bytes;
    pub use tokio::{
        runtime::Handle,
        sync::{
            broadcast::{
                channel, error::SendError as Ber, error::SendError as Ser, Receiver as Brx,
                Sender as Btx,
            },
            oneshot, watch, Barrier, Mutex, OnceCell, RwLock,
        },
        task::{AbortHandle, JoinHandle},
    };

    //
    #[derive(Debug, Clone)]
    pub struct MatriSync(pub Btx<()>, pub Brx<()>);

    #[derive(Debug, Clone)]
    pub struct ProtocolMessage {
        pub header: Node,
        pub message: Bytes,
    }

    pub struct Sigma {
        pub main_receiver: Brx<ProtocolMessage>,
        pub extra_receivers: Option<Vec<Btx<ProtocolMessage>>>,
        pub sender: Btx<ProtocolMessage>,
        pub shutdown_receiver: oneshot::Receiver<()>,
        _shutdown_signal: watch::Sender<()>,
    }

    pub struct SigmaHandle {
        pub shutdown_sender: oneshot::Sender<()>,
        pub shutdown_signal: watch::Receiver<()>,
    }
}
///////////////////////////////////////////////////////////////////////////////

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
