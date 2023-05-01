use std::any::TypeId;
use tokio::runtime::Handle;

use crate::craft::*;

// Matrisync connection utility - Broadcast MPMC channel
impl<T> Craftable for Matrisync<T>
// Matrisync<T> (    //
//     pub BCtx<T>,  // Broadcast Transmitter
//     pub BCrx<T>,  // Broadcast Reciever
// )   // The type passed in generically is what is sent
//     // over the channels. It must be Clone ready
where
    T: Clone,
{
    fn init() -> Matrisync<T> {
        let (tx, rx) = b_chan(32);
        Matrisync(tx, rx)
    }
    fn conf<A>(&mut self) -> Self {
        unimplemented!()
    }
    fn lock<A>(&mut self) -> Self {
        unimplemented!()
    }
    fn free<A>(&mut self) -> Self {
        unimplemented!()
    }
}
impl<T> Clone for Matrisync<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Matrisync(self.0.clone(), self.0.subscribe())
    }
}
