use std::any::Any;
use std::hash::{Hash, Hasher};
//use std::collections::HashMap;

// fn this() {
//         let mut map = HashMap::<Box<dyn DynHash>, _>::new();

//         fn  new<T: DynHash>(value: T) -> Box<dyn DynHash> {
//             Box::new(value)
//         }

//     assert!(new(1u8) == new(1u8));
//     assert!(new(1u8) != new(2u8));
//     assert!(new(1u8) != new(1i8));
//     assert!(new(1u8) != new("hello"));

//         map.insert(new(-1i32), "-1i32");
//     map.insert(new("hello"), "the string \"hello\"");
//     map.insert(new(4u128), "4u128");
//     map.insert(new(()), "()");

//         assert_eq!(map.remove(&"hello" as &dyn DynHash), Some("the string \"hello\""));
// }

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
