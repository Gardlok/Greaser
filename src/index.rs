// Async Global Multi Indexer
// Ex of needs
// fn adduser() { let (user_id, workspace_id) = Indexer.get_next_for(user, workspace)}
// Basic (non async) Multi Indexer keyed by type

pub mod NodeTally {
    use super::SerialInt::*;
    use hashbrown::HashMap;

    pub struct Tally {
        data: HashMap<u8, SerialGenerator>,
        tally: SerialGenerator<u8>,
    }
    impl Tally {
        pub fn new() -> Tally {
            Tally {
                data: HashMap::new(),
                tally: SerialGenerator::<u8>::new(),
            }
        }
        pub fn next(&mut self, rtid: Option<usize>) -> u8 {
            // If a RT id is not provided, create one equal to the next value in RT index. If
            // the index was empty, be the first entry
            if rtid.is_some() {
                self.data.entry(rtid.unwrap()).and_modify(|v| v.generate())
            } else {
                self.data
                    .values_mut()
                    .for_each(|&mut v| v = SerialGenerator::<u8>::new())
            }
        }
        pub fn reset(&mut self, rtid: Option<u8>) {
            // If a RT id is not provided, reset all the indices for each RT id, however
            // each RT will retain it's id.
            if rtid.is_some() {
                self.data
                    .entry(rtid.unwrap())
                    .and_replace_entry_with(|_, v| Some(SerialGenerator::<u8>::new()));
            } else {
                self.data
                    .values_mut()
                    .for_each(|&mut v| v = SerialGenerator::<u8>::new())
            }
        }
        pub fn resetall(&mut self) {
            self.data.clear()
        }
    }
}

pub mod TypeTally {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    // Basic (non async) Multi Indexer keyed by type
    pub struct Tally {
        data: HashMap<TypeId, usize>,
    }
    impl Tally {
        pub fn new() -> Tally {
            Tally {
                data: HashMap::new(),
            }
        }
        pub fn next<T: Any + 'static>(&mut self) -> usize {
            let t = TypeId::of::<T>();
            let t2 = self
                .data
                .entry(t)
                .and_modify(|x| *x += 1)
                .or_insert_with_key(|k| 0);
            t2.to_owned()
        }
        pub fn reset<T: Any + 'static>(&mut self) {
            let t = TypeId::of::<T>();
            *self.data.entry(t).or_insert_with_key(|k| 0) = 0;
        }
        pub fn resetall(&mut self) {
            self.data.clear()
        }
    }
}
// Testing:
// let mut testmap = crate::TypeTally::new();
// let first: String = "Hello".to_string();
// let second: i8 = 5;
// let third: bool = true;
// assert_eq!(testmap.next::<newb<_>>(), 0);
// assert_eq!(testmap.next::<newb<_>>(), 1);
// assert_eq!(testmap.next::<newb<_>>(), 2);
// assert_eq!(testmap.next::<pro<_>>(), 0);
// assert_eq!(testmap.next::<pro<_>>(), 1);
// assert_eq!(testmap.next::<newb<_>>(), 3);

/////////////////////////////////////////////////////////////////////
// HashMap allow any type as keys
mod AnyMap {
    use std::any::{Any, TypeId};
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
    ////////////
    //
    // Keys can be any type while values have share the same type
    struct DTKHashMap<T>(hashbrown::HashMap<Box<dyn DynHash>, T>);

    //let mut map = HashMap::<Box<dyn DynHash>, _>::new();
    // type newb: DynHash = ();
    // type pro<T> = T;

    // fn new<T: DynHash>(value: T) -> Box<dyn DynHash> {
    //     Box::new(value)
    // }

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

mod SerialInt {
    use std::{
        fmt,
        fmt::{Display, Formatter},
        mem::replace,
    };

    pub trait Serial: PartialEq {
        const START: Self;
        fn next_increment(&self) -> Self;
        fn prev_increment(&self) -> Self;
        fn is_max_value(&self) -> bool;
    }

    macro_rules! impl_serial {
            ($($t:ty),+ $(,)?) => {
                $(
                impl Serial for $t {
                    const START: Self = Self::MIN;
                    fn next_increment(&self) -> Self {
                            self.saturating_add(1)
                        }
                    fn prev_increment(&self) -> Self {
                            self.saturating_sub(1)
                        }
                    fn is_max_value(&self) -> bool {
                            self == &Self::MAX
                        }
                }
            )+
        }
    }
    impl_serial!(u8, u16, u32, u64, u128, usize);

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]

    pub struct SerialGenerator<T: Serial = u8> {
        value: T,
    }

    impl<T: Serial> SerialGenerator<T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_init_value(value: T) -> Self {
            SerialGenerator { value }
        }

        pub fn generate(&mut self) -> T {
            let next = self.value.next_increment();

            replace(&mut self.value, next)
        }

        pub fn previous(&self) -> Option<T> {
            if self.value == T::START {
                None
            } else {
                Some(self.value.prev_increment())
            }
        }

        pub fn is_at_max(&self) -> bool {
            self.value.is_max_value()
        }
    }

    impl<T: Serial, U: Serial + From<T>> From<T> for SerialGenerator<U> {
        fn from(other: T) -> Self {
            SerialGenerator::with_init_value(other.into())
        }
    }

    impl<T: Serial> Default for SerialGenerator<T> {
        fn default() -> Self {
            SerialGenerator { value: T::START }
        }
    }

    impl<T: fmt::Debug + Display + Serial> Display for SerialGenerator<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<T: Serial> Iterator for SerialGenerator<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.is_at_max() {
                None
            } else {
                let next_value = self.generate();
                Some(next_value)
            }
        }
    }
}
