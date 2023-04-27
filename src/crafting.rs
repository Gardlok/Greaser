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
    pub type Storage<T> = std::collections::HashMap<DataCraft::EdgeInfo, T>;
    // Wrapper of Metadata for transfers among and of Nodes
    pub type Unit<DATA> = (DataCraft::EdgeInfo, DATA);
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
    // Specific Attributes Info
    #[derive(PartialEq, Eq, Hash)]
    pub enum NodeInfo {
        Noid(u8), // Node ID
        Quid(u8), // Quick ID
        Ntid(u8), // Type ID
        Stid(u8), // State ID
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum EdgeInfo {
        Liid(u8), // Edge ID
        Scid(u8), // Scope ID
        Paid(u8), // Pattern ID
        Ltid(u8), // Type ID
    }
    #[derive(PartialEq, Eq, Hash)]
    pub enum DataInfo {
        Daid(u8), // Data ID
        Scid(u8), // Scope ID
        Foid(u8), // Format ID
        Dtid(u8), // Type ID
    }

    // Data

    // Node
}

pub mod Craft {
    use super::*;

    pub enum Rudiment {
        Node,
        Edge,
        Data,
    }

    pub trait Craftable<R> {
        type Item<T>;
        type Rudi;
        fn init<T>(/*Item */) -> Result<Self::Item<R>, ()>;
        fn conf<T>(&mut self) -> Self::Item<T>;
        fn lock<T>(&mut self) -> Self::Item<T>;
        fn free<T>(&mut self) -> Self::Item<T>;
    }

    impl<R> Craftable<R> for Node {
        fn init<T>() -> Result<Self, ()> {}
        fn conf<T>(self) -> Self {
            self
        }
        fn lock<T>(self) -> Self {
            self
        }
        fn free<T>(self) -> Self {
            self
        }
    }
    pub trait Tinkerable<R> {
        type Item<T>;
        type Rudi;
        fn def(self, attr: &[A]) -> Result<Self::Item, ()>;
        fn __def(self, attr: &[A]) -> Result<Self::Item, ()>;
        fn __flip(self, attr: &[A]) -> Result<Self::Item, ()>;
        fn __incr(self, attr: &[A]) -> Result<Self::Item, ()>;

    impl<R> Tinkerable<R> for Node {
        fn def(&mut self, attr: &[A]) -> Result<Self::Item, ()> {self}
        fn __def(self, attr: &[A]) -> Result<Self::Item, ()> {self}
        fn __flip(self, attr: &[A]) -> Result<Self::Item, ()> {self}
        fn __incr(self, attr: &[A]) -> Result<Self::Item, ()> {self}
    }


    #[derive(PartialEq, Eq)]
    pub struct NodeStruct<S, T>(
        // S: Scope T: Type
        OnceCell<u32>,
        PhantomData<(S, T)>,
    );
    #[derive(PartialEq, Eq)]
    struct EdgeStruct<P, S, T>(
        // P: Pattern S: Scope T: Type //
        u64,
        PhantomData<(P, S, T)>,
    );
    #[derive(PartialEq, Eq)]
    struct DataStruct<P, C>(
        // P: Priority C: Class
        u16,
        PhantomData<(P, C)>,
    );

    pub type Node = NodeStruct<(), ()>;
    pub type Edge = EdgeStruct<(), (), ()>;
    pub type Data = DataStruct<(), ()>;
    fn trythis() {
        let a = Node::Generate(10);
    }
    trait IdCraft {
        fn GenNewU32<Node>(id: u32) -> u32 {}
        // fn GenNewU32(id: u32) -> u32 {
        //     Nodef(OnceCell::new_with(id), PhantomData)
        // }
    }
}

// pub trait Craftable<R> {
//     type Item<T>;
//     type Rudi;
//     fn init<T>(/*Item */) -> Result<Self::Item<R>, ()>;
//     fn conf<T>(&mut self) -> Self::Item<T>;
//     fn lock<T>(&mut self) -> Self::Item<T>;
//     fn free<T>(&mut self) -> Self::Item<T>;
// }

// impl<R> Craftable<R> for Node {
//     fn init<T>() -> Result<Self, ()> {}
//     fn conf<T>(self) -> Self::Item<R> {}
//     fn lock<T>(self) -> Self::Item<R> {}
//     fn free<T>(self) -> Self::Item<R> {}
// }
