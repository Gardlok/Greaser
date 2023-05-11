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
mod tests {

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
