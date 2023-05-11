use std::any::TypeId;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;
use tokio_util::io::StreamReader;

use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};
use crate::MAG_NUM;

impl Clone for MatriStream
// where
//     Self: Copy,
{
    fn clone(&self) -> Self {
        MatriStream(self.0.clone(), self.0.subscribe())
    }
}

impl MatriStream {
    //
    pub async fn new() -> MatriStream {
        let cap = MAG_NUM.saturating_mul(100).into();
        let (tx, rx) = channel(cap);
        MatriStream(tx, rx)
    }
    //
    pub async fn send<T: Send>(&mut self, content: T) -> Result<usize, Ser<()>> {
        self.0.send(())
    }
    //
    pub async fn recv(&mut self, content: &dyn tokio::io::AsyncBufRead) -> Result<(), ()> {
        &self.1;
        Ok(())
    }
}
