/*

Biblio:
    indicating book or books

BiblioCraft - Maps, indices, ledgers, etc.
==========================================
    Synaptics
    NestIndex
    TypeTally
    SerialInt
    Matridex

*/

pub mod Lexicon {
    use polars::prelude::*;
    //
    pub trait Lexical {
        async fn lexicon() -> DataFrame;
    }
}

pub mod NestIndex {
    use super::SerialInt::*;
    use hashbrown::HashMap;

    // Indexing with two layers, a parent layer and a child layer.
    pub struct NestIndex {
        data: HashMap<u8, SerialGenerator>,
        tally: SerialGenerator<u8>,
    }
    impl NestIndex {
        pub fn new() -> NestIndex {
            NestIndex {
                data: HashMap::new(),
                tally: SerialGenerator::<u8>::new(),
            }
        }
        // Retrieves the next available parent id
        pub fn next(&mut self) -> (u8, u8) {
            let index = self.tally.generate();
            self.data.insert(index, SerialGenerator::new());
            self.next_in(index)
        }
        pub fn next_in(&mut self, parentid: u8) -> (u8, u8) {
            // If a parent id is not provided, create one equal to the next value in parent index. If
            // the index was empty, be the first entry
            let gen = self.data.get_mut(&parentid);
            if gen.is_some() {
                (parentid, gen.unwrap().generate())
            } else {
                let mut gen = SerialGenerator::<u8>::new();
                let childid = gen.generate();
                self.data.insert(parentid, gen);
                (parentid, childid)
            }
        }
        pub fn reset(&mut self, parentid: Option<u8>) {
            // If a parent id is not provided, reset all the indices for each parent id, however
            // each parent will retain it's id. If one is provided, only that parent id will
            // reset it's child id's.
            if parentid.is_some() {
                self.data
                    .entry(parentid.unwrap())
                    .and_replace_entry_with(|_, v| Some(SerialGenerator::<u8>::new()));
            } else {
                self.data.values_mut().map(|_| SerialGenerator::<u8>::new());
            }
        }
        pub fn resetall(&mut self) {
            self.data.clear()
        }
    }
}

// Basic (non async) Multi Indexer keyed by type
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
    mod tests {
        use super::Tally;
        #[test]
        fn typetally() {
            let mut testmap = Tally::new();
            let first: String = "Hello".to_string();
            let second: i8 = 5;
            let third: bool = true;
            assert_eq!(testmap.next::<u8>(), 0);
            assert_eq!(testmap.next::<u8>(), 1);
            assert_eq!(testmap.next::<u8>(), 2);
            assert_eq!(testmap.next::<u16>(), 0);
            assert_eq!(testmap.next::<u16>(), 1);
            assert_eq!(testmap.next::<u8>(), 3);
        }
    }
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
    // BUG!
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

/////////////////////////////////////////////////////////////////////////////////////
// Layered Indices
//   1) Edge Layer |  ( Sender & Reciever  -  Node &| Scope )
//   2) Data Layer |  ( Class & Priority of Content         )
//   3) Exec Layer |  ( Ownership, Allocation, Handling     )
//   4) Rins Layer |  ( Reporting, Cleanup, Dropping        )
//
/////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////
mod Matridex {
    /*
        Instead of commonly used indices or even conventional thinking of "Indexing" is...
    */
    use bitvec::prelude::*;
    use enum_iterator;
    use polars::frame::DataFrame;
    use polars::prelude::*;

    enum Synaptic {
        Node, // 2 - Node type
        Role, // 2 - Role class
        Life, // 2 - Lifetime expectancy
        Stat, // 2 - State of overall status
        Edge, // 2 - Edge state
    }

    /*

    Node Attr (Actions, Positions, Observations)
    ============================================
    What is needed to infer the purpose of a process?
    This purpose will be used as the identification
    of the process itself. It's purpose and relation
    to other processes should be more important than
    humanfriendly sought concepts, such as linear
    indexing. Instead of identification being a value
    in a seperate data set and series of abstractions,
    I consider the notion of either: ID defined by
    purpose or removal of the need to ID anyway.



    When (Timestamps)
        Transferring of data ownership
        Manipulation of data
        Generation of data
        Alterations of passively derived attributes
        Edge activity

    What (Data)
        ammount of contextual data owned
        ammount of operational data owned
        Associated data types

    Where (Scopes)
        State instances affected
        signatures of co-op processes (synapses)


    -------------------------------------------------
    impl SynapticWeight {
        fn w(self) -> usize {
            match self {
                Node => 2,
                Role => 2,
                Life => 2,
                Stat => 2,
                Scop => 2,
                Edge => 2,
            }
        }
    }
    */
}
