//
// Node, Edge, and Data Crafting
use crate::{Data, Edge, Node};
use std::marker::PhantomData;
//
//
/////////////////////////////////////////////////////////////////////////
pub mod EdgeCraft {

    use super::*;

    use futures::{
        stream::FuturesOrdered as ord_futs,     //
        stream::FuturesUnordered as unord_futs, //
        Future,                                 //
        FutureExt,                              //
        StreamExt,                              //
    };
    use tokio::sync::broadcast::{channel as b_chan, Receiver as BCrx, Sender as BCtx};
    use tokio::sync::mpsc::{channel as mpsc_chan, Receiver as MPSCrx, Sender as MPSCtx};
    use tokio::sync::oneshot::{channel as os_chan, Receiver as OSrx, Sender as OStx};
    use tokio::sync::watch::{channel as w_chan, Receiver as Wrx, Sender as Wtx};
    use tokio::sync::{Barrier, OnceCell};

    // Sub
    pub mod Create {
        use super::*;
        use b_chan as broadcast;
        use mpsc_chan as mpsc;
        use os_chan as oneshot;
        use w_chan as watch;
    }

    // Attribute Variants
    #[derive(PartialEq, Eq, Hash)]
    pub enum Pattern<T> {
        Mesh,
        Pair,
        XPair,
        _Attr(PhantomData<T>),
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum State<T> {
        Staging,
        Active,
        Cancelled,
        _Attr(T),
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum EdgeType {
        Message,
        Poll,
        Asset,
    }

    // Type management
    pub type SharedCast<T> = (BCtx<T>, BCrx<T>);
    pub type Single<T> = (MPSCtx<T>, MPSCrx<T>);
    pub type OneShot<T> = (OStx<T>, OSrx<T>);
    pub type Watch<T> = (Wtx<T>, Wrx<T>);

    // Storage for handles
    //pub type Storage<T> = std::collections::HashMap<DataCraft::EdgeInfo, T>;
    // Wrapper of Metadata for transfers among and of Nodes
    // pub type Unit<DATA> = (DataCraft::EdgeInfo, DATA);
    // Error Handling
    #[derive(thiserror::Error, Debug)]
    pub enum EdgeError<T> {
        #[error("Error Sending Data")]
        SendFail_mpsc(#[from] tokio::sync::mpsc::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFail_os(#[from] tokio::sync::oneshot::error::RecvError),
        #[error("Error Sending Data")]
        SendFail_bs(#[from] tokio::sync::broadcast::error::SendError<T>),
        #[error("Error Recv Data")]
        RecvFail_bs(#[from] tokio::sync::broadcast::error::RecvError),
    }
}

//////////////////////////////////////////////////////////////////
pub mod NodeCraft {
    use super::*;

    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum State {
        Setup,
        Init,
        Run,
        Diag,
        Stop,
        End,
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum NodeType {
        Runtime,
        Task,
        Shadow,
    }
    // Error Handling for Node crafting
}

////////////////////////////////////////////////////////////////////
pub mod DataCraft {
    use super::*;

    // General Attributes
    #[derive(PartialEq, Eq, Hash)]
    pub enum Priority<T> {
        Low,
        Normal,
        High,
        Urgent,
        _Type(PhantomData<T>),
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum Class<T> {
        Bit,
        Normal,
        Stream,
        Static,
        _Type(PhantomData<T>),
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum Label<T> {
        Scope(String),
        Name(String),
        Description(String),
        _Type(PhantomData<T>),
    }

    /////////////////////////////////////////////////////////////
}

pub mod Craft {
    use super::*;

    pub enum Element {
        Node,
        Edge,
        Data,
    }

    pub trait Craftable
    where
        Self: Sized,
    {
        fn init<T>(/*Item */) -> Result<Self, ()>;
        // fn conf<T>(&mut self) -> Self;
        // fn lock<T>(&mut self) -> Self;
        // fn free<T>(&mut self) -> Self;
    }

    pub trait Tinkerable<Elem>
    where
        Self: Sized,
    {
        type Elem<Attr>;
        fn def<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
        fn __def<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
        fn __flip<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
        fn __incr<Attr>(self, attr: &[Attr]) -> Result<Self, ()>;
    }
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

// #[derive(PartialEq, Eq, Hash)]
// pub enum NodeInfo {
//     Noid(u8), // Node ID
//     Quid(u8), // Quick ID
//     Ntid(u8), // Type ID
//     Stid(u8), // State ID
// }
// #[derive(PartialEq, Eq, Hash)]
// pub enum EdgeInfo {
//     Liid(u8), // Edge ID
//     Scid(u8), // Scope ID
//     Paid(u8), // Pattern ID
//     Ltid(u8), // Type ID
// }
// #[derive(PartialEq, Eq, Hash)]
// pub enum DataInfo {
//     Daid(u8), // Data ID
//     Scid(u8), // Scope ID
//     Foid(u8), // Format ID
//     Dtid(u8), // Type ID
// }
