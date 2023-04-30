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
#[cfg(test)]
mod tests {
    use super::*;
    // use crate::*;

    #[test]
    fn tester() {
        //let mut map = HashMap::<Box<dyn DynHash>, _>::new();

        fn new<T: DynHash>(value: T) -> Box<dyn DynHash> {
            Box::new(value)
        }
        let mut testmap = crate::TypeTally::new();
        let first: String = "Hello".to_string();
        let second: i8 = 5;
        let third: bool = true;
        assert_eq!(testmap.next::<i8>(), 0);
        assert_eq!(testmap.next::<i8>(), 1);
        assert_eq!(testmap.next::<i8>(), 2);
        assert_eq!(testmap.next::<i16>(), 0);
        assert_eq!(testmap.next::<i16>(), 1);
        assert_eq!(testmap.next::<i8>(), 3);

        // assert!(false);
        // assert!(new(1u8) != new(2u8));
        // assert!(new(1u8) != new(1i8));
        // assert!(new(1u8) != new("hello"));
        // map.insert(new(-1i32), "-1i32");
        // map.insert(new("hello"), "the string \"hello\"");
        // map.insert(new(4u128), "4u128");
        // map.insert(new(49u128), "4u128");
        // map.insert(new(()), "()");
        // enum TTT {
        //     First(u8),
        //     Second,
        // }
        // struct SSS {};
        // let s = SSS {};
        // assert_eq!(map.get(&6u128 as &dyn DynHash), Some(&"4u128"));
        // assert_eq!(map.get(&5u128 as &dyn DynHash), Some(&"4u128"));
        // assert_eq!(
        //     map.remove(&"hello" as &dyn DynHash),
        //     Some("the string \"hello\"")
        // );
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
