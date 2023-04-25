
use futures::{
    stream::FuturesOrdered as ord_futs, stream::FuturesUnordered as unord_futs, Future, StreamExt,
};

use std::collections::HashMap;

use tokio::sync::broadcast::{
    channel as bc_channel, Receiver as BroadCastRx, Sender as BroadCastTx,
};
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
///////////////




pub async fn test_8(i: usize) -> usize {
    i
}

pub async fn test_7(mut rx: BroadCastRx<BroadCastTx<usize>>) -> usize {
    let tx = rx.recv().await;
    let tx = tx.unwrap();
    let _ = tx.send(0);
    0
}

pub struct Prepper {
    name: String,
    funcs: unord_futs<DynFut<usize>>,
}
//////////////////////////////////////////////
impl Prepper {
    pub async fn test_fn(mut self) -> usize {
        self.funcs.next().await.unwrap()
    }
    pub fn test_grp(self) {
        let (tx, rx) = bc_channel(5);
        let (tx2, _) = bc_channel(5);
        self.funcs.push(Box::pin(test_8(0)));
        self.funcs.push(Box::pin(test_7(rx)));
        let _ = tx.send(tx2);
    }
}
/










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
        let eng = Engine::start();
    }
}
