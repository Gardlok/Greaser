use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct DataStruct<P, C>(
    // P: Priority C: Class
    u16,
    PhantomData<(P, C)>,
);

///////////////////////////////////////////////////////////////////
// Dyn Dispatching
//
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

mod tests {
    use crate::{DataCraft::*, EdgeCraft::*};
    use hashbrown::{HashMap, HashSet};

    use super::DynHash;
    //
    type DHashMap = HashMap<Box<dyn DynHash>, Box<dyn DynHash>>;
    type DHashSet = HashSet<Box<dyn DynHash>>;
    fn new<T: DynHash>(value: T) -> Box<dyn DynHash> {
        Box::new(value)
    }

    #[test]
    fn dyn_hashset() {
        let mut hset = DHashSet::new();
        assert!(hset.insert(new(Matridex::new(0u8))));
        assert!(hset.insert(new(EdgeType::Buffr)));
        assert_eq!(EdgeType::Buffr, hset.get(&EdgeType::Buffr).unwrap());
    }
    #[test]
    fn dyn_hashmap() {
        let mut hmap = DHashMap::new();
        assert!(hmap.insert(Matridex::new(0u8), EdgeType::Buffr));
        assert_eq!(EdgeType::Buffr, hmap.get(&Matridex::new(0u8)).unwrap());
    }
}

//let mut map = HashMap::<Box<dyn DynHash>, _>::new();
// type newb: DynHash = ();
// type pro<T> = T;

// assert!(false);
// assert!(new(1u8) != new(1i8));
// map.insert(new(-1i32), "-1i32");
// map.insert(new("hello"), "the string \"hello\"");
// map.insert(new(49u128), "4u128");
// map.insert(new(()), "()");
// assert_eq!(map.get(&6u128 as &dyn DynHash), Some(&"4u128"));
// assert_eq!(
//     map.remove(&"hello" as &dyn DynHash),
//     Some("the string \"hello\"")
// );
