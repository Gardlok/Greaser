use std::any::TypeId;
use tokio::runtime::Handle;

use crate::craft::EdgeCraft::*;

impl<T> Clone for Matrisync<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Matrisync(self.0.clone(), self.0.subscribe(), RwLock::from(self.2))
    }
}

impl<T> Matrisync<T>
where
    T: Clone,
{
    //
    pub async fn new() -> Matrisync<T> {
        let (tx, rx) = b_chan(32);
        Matrisync(tx, rx, Matridex::new(hashbrown::HashSet::new()))
    }
    //
    pub async fn send(self, content: T) -> Result<usize, EdgeError<T>> {
        self.0.send(content).map_err(EdgeError::SendFailBC)
    }
    //
    pub async fn recv(&mut self, content: T) -> Result<T, EdgeError<T>> {
        self.1.recv().await.map_err(EdgeError::RecvFailBC)
    }
}
