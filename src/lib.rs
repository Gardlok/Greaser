use tokio::sync::broadcast::{
    channel as bc_channel, Receiver as BroadCastRx, Sender as BroadCastTx,
};
use tokio::sync::watch::{channel as w_channel, Receiver as WatchRx, Sender as WatchTx};
use tokio::sync::Barrier;
use tokio::sync::OnceCell;

use tokio::runtime::{Builder, Handle as RtHandle, Runtime};
use tokio::task::{spawn, AbortHandle, JoinHandle, JoinSet};
use tokio::task_local;
use tokio::time::{Duration, Instant};

use std::collections::HashMap;

//  Sync and Messaging /////////////////////////////////////////////

//    >---> Tokio  MPMC
//    >---> Watch  SPMC

pub struct Matrisync<T> {
    id: usize,
    node: Node,
    state: State,
    idle: bool,
    j_handle: Option<JoinHandle<T>>,
    a_handle: Option<AbortHandle>,
    t_handle: Option<RtHandle>,
}

enum Msg {
    Push,
    Pull,
    Poll,
}

//  Thread and Task Running ////////////////////////////////////////////////
pub struct Matrices {
    nodes: HashMap<usize, Node>,
}

impl Matrices {
    pub fn add(mut self, node: Node) -> Result<(), ()> {
        self.nodes.insert(0, node);
        Ok(())
    }
}

// Node
pub struct Node {
    name: String,
    node_type: NodeType,
}

impl Node {
    fn build(self) -> Result<Runtime, ()> {
        Builder::new_current_thread()
            .thread_name(self.name)
            .on_thread_start(|| ()) // TODO!
            .on_thread_stop(|| ()) // TODO!
            .on_thread_park(|| ()) // TODO!
            .on_thread_unpark(|| ()) // TODO!
            .enable_time()
            .enable_io()
            .start_paused(true)
            .build()
            .map_err(|_| ()) // TODO!
            .map(|rt| rt) // TODO!
    }
    fn get_handle(self) -> RtHandle {
        unimplemented!()
        // match self {
        // RT::Compendium => ,
        // RT::Space => ,
        // RT::Ascension => ,
        // RT::Matrices => ,
        // };
    }
}

enum NodeType {
    Thread,
    Task,
}

enum State {
    Init,
    Run,
    Diag,
    Stop,
    End,
}

//////////////////////////////////////////////////
// Go time!
/////////////////
mod Engine {
    use super::*;

    pub fn start() -> Matrices {
        let mut nodes = HashMap::new();
        Matrices { nodes }
    }

    pub fn get_ready() -> () {
        loop {}
    }

    pub fn get_set() -> () {
        ()
    }

    macro_rules! GO {
        () => {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
        let eng = Engine::start();
    }
}
