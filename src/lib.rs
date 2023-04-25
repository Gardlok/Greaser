#[allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    unused_imports,
    unsafe_op_in_unsafe_fn
)]
use futures::task::FutureObj;
use futures::FutureExt;
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

use futures::{
    stream::FuturesOrdered as ord_futs, stream::FuturesUnordered as unord_futs, Future, StreamExt,
};

use std::collections::HashMap;
use std::fmt;
use std::pin::Pin;
mod test;

/////////////////////////////////////////////
// Types
type RunTime<T> = ::std::pin::Pin<Box<dyn Send + Future<Output = T>>>;
type Task<T> = ::std::pin::Pin<Box<dyn Send + Future<Output = T>>>;

/////////////////////////////////////////////
// Primary logic and work flow
pub struct Matrices<T> {
    // Primary cfg, Running data, Methods, and Node Handles.
    // Responsible for Node creation, oversight, and cleanup.
    //
    nodes: HashMap<usize, Vec<Node>>,
    funcs: HashMap<RunTime<T>, Vec<Task<T>>>,
}

impl<T> Matrices<T>
where
    T: Send,
{
    pub fn add(mut self, node: Node) -> Result<(), ()> {
        self.nodes
            .entry(0)
            .and_modify(|v| v.push(node))
            .or_insert(vec![node]);
        Ok(())
    }
}

// Build Node
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum ConfigurationProgress<T> {
    NotSet(T),
    AllSet(T),
}
type CP<T> = ConfigurationProgress<T>;
impl<T> ConfigurationProgress<T> {
    const fn is_set(&self) -> bool {
        matches!(*self, CP::AllSet(_))
    }
    pub fn is_set_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            CP::NotSet(x) => f(x),
            CP::AllSet(x) => f(x),
        }
    }
    pub const fn as_ref(&self) -> CP<&T> {
        match *self {
            CP::AllSet(ref x) => CP::AllSet(x),
            CP::NotSet(ref x) => CP::NotSet(x),
        }
    }
    pub const fn as_mut(&mut self) -> CP<&T> {
        match *self {
            CP::AllSet(ref mut x) => CP::AllSet(x),
            CP::NotSet(ref mut x) => CP::NotSet(x),
        }
    }
    pub const fn as_pin_ref(self: Pin<&Self>) -> CP<Pin<&T>> {
        match Pin::get_ref(self).as_ref() {
            CP::AllSet(x) => unsafe { CP::AllSet(Pin::new_unchecked(x)) },
            CP::NotSet(x) => unsafe { CP::NotSet(Pin::new_unchecked(x)) },
        }
    }
    pub const fn as_pin_mut(self: Pin<&mut Self>) -> CP<Pin<&mut T>> {
        unsafe {
            match Pin::get_unchecked_mut(self) {
                CP::AllSet(x) => CP::AllSet(Pin::new_unchecked(x)),
                CP::NotSet(x) => CP::NotSet(Pin::new_unchecked(x)),
            }
        }
    }
}

impl<T> std::fmt::Display for ConfigurationProgress<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CP::AllSet(T) => Ok(print!("{:?}", T)),
            CP::NotSet(T) => Ok(print!("{:?}", T)),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    name: CP<String>,
    id: usize,
    node_type: NodeType,
    scope: String,
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

#[derive(Debug)]
enum NodeType {
    RunTime,
    Task,
}

trait NT_RunTime
where
    Self: Sized,
{
    fn mode(self) -> NodeType {
        NodeType::RunTime
    }
}
trait NT_Task
where
    Self: Sized,
{
    fn mode() -> NodeType {
        NodeType::Task
    }
}

#[derive(Debug)]
enum State {
    Init,
    Run,
    Diag,
    Stop,
    End,
}

//  Sync and Messaging /////////////////////////////////////////////

//    >---> Tokio  MPMC
//    >---> Watch  SPMC

#[derive(Debug)]
pub struct Matrisync<T> {
    id: usize,
    node: Node,
    state: State,
    idle: bool,
    j_handle: Option<JoinHandle<T>>,
    a_handle: Option<AbortHandle>,
    t_handle: Option<RtHandle>,
}

#[derive(Debug)]
enum Msg {
    Push,
    Pull,
    Poll,
}

//////////////////////////////////////////////////
// Go time!
/////////////////
mod Engine {
    use super::*;

    pub fn start<T>() -> Matrices<T> {
        let mut nodes = HashMap::new();
        let mut funcs = HashMap::new();
        Matrices { nodes, funcs }
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
