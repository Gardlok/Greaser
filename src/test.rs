// use futures::{
//     stream::FuturesOrdered as ord_futs, stream::FuturesUnordered as unord_futs, Future, StreamExt,
// };

// use std::collections::HashMap;

// use tokio::sync::broadcast::{
//     channel as bc_channel, Receiver as BroadCastRx, Sender as BroadCastTx,
// };

// use crate::*;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use hashbrown::HashMap;
    // use crate::thashmap::
    use std::collections::hash_map::DefaultHasher;
    use std::marker::PhantomData;

    use std::any::Any;
    use std::hash::{Hash, Hasher};

    pub trait DynEq: Any {
        fn dyn_eq(&self, other: &dyn DynEq) -> bool;

        fn as_any(&self) -> &dyn Any;
    }

    pub trait DynHash: DynEq {
        fn dyn_hash(&self, hasher: &mut dyn Hasher);

        fn as_dyn_eq(&self) -> &dyn DynEq;
    }

    impl<H: Eq + Any> DynEq for H {
        fn dyn_eq(&self, other: &dyn DynEq) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<H>() {
                self == other
            } else {
                false
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl<H: Hash + DynEq> DynHash for H {
        fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
            H::hash(self, &mut hasher)
        }

        fn as_dyn_eq(&self) -> &dyn DynEq {
            self
        }
    }

    impl PartialEq for dyn DynHash {
        fn eq(&self, other: &dyn DynHash) -> bool {
            self.dyn_eq(other.as_dyn_eq())
        }
    }

    impl Eq for dyn DynHash {}

    impl Hash for dyn DynHash {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.dyn_hash(hasher)
        }
    }

    #[test]
    fn tester() {
        use std::collections::HashMap;

        let mut map = HashMap::<Box<dyn DynHash>, _>::new();

        fn new<T: DynHash>(value: T) -> Box<dyn DynHash> {
            Box::new(value)
        }

        assert!(new(1u8) == new(1u8));
        assert!(new(1u8) != new(2u8));
        assert!(new(1u8) != new(1i8));
        assert!(new(1u8) != new("hello"));

        map.insert(new(-1i32), "-1i32");
        map.insert(new("hello"), "the string \"hello\"");
        map.insert(new(4u128), "4u128");
        map.insert(new(()), "()");

        assert_eq!(
            map.remove(&"hello" as &dyn DynHash),
            Some("the string \"hello\"")
        );
    }
}

//     use std::{
//         cmp::Ordering,
//         hash::{Hash, Hasher},
//         mem::{size_of, transmute_copy},
//         ops::Deref,
//         //raw::TraitObject,
//     };

//     impl_cmp_dyn_traits! {
//         trait DynPartialEq for PartialEq {
//                 dyn_eq -> bool { eq, false }
//                 dyn_ne -> bool { ne, true }
//             }

//         trait DynEq: DynPartialEq for Eq {}

//         trait DynPartialOrd: DynPartialEq for PartialOrd {
//                 dyn_partial_cmp -> Option<Ordering> { partial_cmp, None }
//                 dyn_le -> bool { le, false }
//                 dyn_lt -> bool { lt, false }
//                 dyn_ge -> bool { ge, false }
//                 dyn_gt -> bool { gt, false }
//         }
//     }

//     #[derive(Clone, Copy)]
//     pub struct CmpDyn<T: ?Sized, Phantom = ()> {
//         _phantom: PhantomData<Phantom>,
//         pub inner: T,
//     }

//     impl<T, Phantom> CmpDyn<T, Phantom> {
//         fn new(value: T) -> Self {
//             CmpDyn {
//                 inner: value,
//                 _phantom: PhantomData,
//             }
//         }
//     }

//     fn eq_box(value: impl 'static + PartialEq) -> CmpDyn<Box<dyn 'static + DynPartialEq>> {
//         CmpDyn::new(Box::new(value))
//     }
//     type Key = CmpDyn<Box<dyn 'static + DynHash<DefaultHasher>>, DefaultHasher>;
//     fn key(value: impl 'static + Eq + Hash) -> Key {
//         CmpDyn::new(Box::new(value))
//     }
//     //let mut map: HashMap<Key, &'static str> = HashMap::new();
// }
// // #[test]
// // fn it_works() {
//     assert!(true);
//     let e = HashMap::new * (newt);
// let eng = Engine::start();
// println!("{:?}", eng);

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
