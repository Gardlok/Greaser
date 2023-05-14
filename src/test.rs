use crate::*;
use tokio::pin;
use tokio_test::*;

///////////////////////////////////
macro_rules! toktest {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
///////////////////////////////////

mod tests1 {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_client(b: &mut Bencher) {
        let (sender, receiver) = broadcast::channel(10);
        let mut cool_factory = CoolFactory::new(sender.clone(), receiver.subscribe());
        let interest = arr2(&[[0, 1, 2], [3, 4, 5]]);
        let client = Client::new(sender.clone(), receiver.subscribe());

        b.iter(|| {
            cool_factory.add_client(interest.clone(), client.clone());
            cool_factory.remove_client(&interest);
        });
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("broadcast channel error: {0}")]
    BroadcastChannelError(#[from] tokio::sync::broadcast::Error),

    #[error("dataframe conversion error: {0}")]
    DataFrameConversionError(#[from] polars::error::PolarsError),

    #[error("client error: {0}")]
    ClientError(String),

    #[error("factory error: {0}")]
    FactoryError(String),

    #[error("serde serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("unknown error")]
    Unknown,
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use ndarray::arr2;
    use polars::prelude::*;

    #[tokio::test]
    async fn protocol_message_creation() {
        let header = arr2(&[[0, 1, 2], [3, 4, 5]]);
        let payload = DataFrame::new(vec![]).unwrap();
        let message = ProtocolMessage::new(header, payload);
        assert_eq!(message.header, arr2(&[[0, 1, 2], [3, 4, 5]]));
        assert_eq!(message.payload, DataFrame::new(vec![]).unwrap());
    }

    #[tokio::test]
    async fn client_send_receive_message() {
        let (sender, receiver) = broadcast::channel(10);
        let client = Client::new(sender.clone(), receiver.subscribe());

        let header = arr2(&[[0, 1, 2], [3, 4, 5]]);
        let payload = DataFrame::new(vec![]).unwrap();
        let message = ProtocolMessage::new(header.clone(), payload.clone());

        client.send_message(message.clone());
        let received_message = client.receive_message().unwrap();

        assert_eq!(message.header, received_message.header);
        assert_eq!(message.payload, received_message.payload);
    }

    #[tokio::test]
    async fn cool_factory_add_remove_client() {
        let (sender, receiver) = broadcast::channel(10);
        let mut cool_factory = CoolFactory::new(sender.clone(), receiver.subscribe());

        let interest = arr2(&[[0, 1, 2], [3, 4, 5]]);
        let client = Client::new(sender.clone(), receiver.subscribe());

        cool_factory.add_client(interest.clone(), client);
        assert!(cool_factory.clients.contains_key(&interest));

        cool_factory.remove_client(&interest);
        assert!(!cool_factory.clients.contains_key(&interest));
    }
}

pub async fn test_9(i: usize) -> usize {
    i
}
pub async fn test_9result(i: usize) -> Result<(), ()> {
    Ok(())
}
pub async fn test_sigma1(sigma: Option<Sigma>, i: usize) -> Result<(), ()> {
    let mut sigma = sigma.unwrap();
    // while let Ok(_) = sigma.matrisync.1.recv().await {
    //     let _ = sigma.matrisync.0.send(());
    // }
    Ok(())
}

pub async fn build_add_exec() -> usize {
    let mut matrix = Matrices::new().await;
    for i in 1..3 {
        let _ = matrix.add(None, Box::pin(test_9result(i))).await;
    }
    // matrix.op(i)
    1
}

#[cfg(test)]
mod tests3 {

    use crate::Matrices;

    use super::*;
    // use crate::*;

    #[test]
    fn test999() {
        assert_eq!(toktest!(test_9(9)), 9);
    }

    #[test]
    fn basis() {
        let matrix = toktest!(Matrices::new());

        assert_eq!((), ());
    }
}

/*///////////////
pub async fn test_8(i: usize) -> usize {
    i
}
impl Prepper {
    pub async fn test_fn(mut self) -> usize {
        self.funcs.next().await.unwrap()
    }
    pub fn test_grp(self) {
        self.funcs.push(Box::pin(test_8(0)));
    }
}*/

/*
Engine
    .fire_up(3)
    .def("Main")
    ._sco("api_serv-primary")
    ._def("load_global_config")
    ._def("logging_runner")
    .def("Data")
    ._sco("api_serv-primary")
    ._def("db_watcher")
    .def("API")
    ._sco("api_serv-secured_layer")
    ._def("direct_chan_to_db")
    .get_ready()
    .get_set(|id_set| { do_something(id_set) })
    .go()
*/
