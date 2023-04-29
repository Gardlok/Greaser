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

////////////////////////////////////////////////
//  A set of runtime/task handles that are
//  clonable among nodes/threads
impl<T> Clone for Handles<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
//
use std::fmt::{Debug, Formatter, Result};
impl<T> Debug for Handles<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Handles")
            .field("RunTime", &self.0)
            .field("All Handles:", &self)
            .field("Task Abort", &self.2)
            .finish()
    }
}
// impl<T> Craftable for Handles<T>
// where
//     T: Clone,
// {
//     fn init() -> Handles<T> {
//         unimplemented!()
//     }
//     fn conf<A>(&mut self) -> Self {
//         unimplemented!()
//     }
//     fn lock<A>(&mut self) -> Self {
//         unimplemented!()
//     }
//     fn free<A>(&mut self) -> Self {
//         unimplemented!()
//     }
// }
// //
// //
