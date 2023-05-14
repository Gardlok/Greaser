//
//
//
//
use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};
use crate::MAG_NUM;

impl Sigma {
    pub fn new(
        main_receiver: Brx<ProtocolMessage>,
        sender: Btx<ProtocolMessage>,
    ) -> (Self, SigmaHandle) {
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let (shutdown_signal, shutdown_watch) = watch::channel(());
        let client = Self {
            main_receiver,
            extra_receivers: None,
            sender,
            shutdown_receiver,
            _shutdown_signal: shutdown_signal,
        };

        let handle = SigmaHandle {
            shutdown_sender,
            shutdown_signal: shutdown_watch,
        };
        (client, handle)
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut main_receiver = self.sender.subscribe();
        let mut extra_receivers = self.extra_receivers.as_ref().map(|receivers| {
            receivers
                .iter()
                .map(|receiver| receiver.subscribe())
                .collect::<Vec<_>>()
        });

        loop {
            tokio::select! {
                Some(message) = main_receiver.recv() => {
                        // Handle message from main receiver
                }
                Some(message) = extra_receivers.as_mut()
                    .and_then(|receivers| receivers.iter_mut()
                        .find_map(|receiver| receiver.recv().ok())) => {
                        // Handle message from extra receivers
                    }
                _ = &self.shutdown_receiver => {
                        println!("Shutdown signal received, stopping client.");
                        break;
                }
            }
        }
        // Perform any necessary cleanup here
        Ok(())
    }
}

impl std::fmt::Debug for Sigma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sigma").finish()
    }
}
impl SigmaHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown_sender.send(());
    }
    pub async fn monitor(&mut self) {
        while self.shutdown_signal.changed().await.is_ok() {}
        println!("Sigma has shut down.");
    }
}

// mod tests {
//     use super::*;

//     async fn test_client() {
//         let runtime = Runtime::new().unwrap();
//         let (tx, rx) = broadcast::channel(100);
//         let client = Sigma::new(rx, tx);
//         // TODO: Add actual tests here
//     }
// }
