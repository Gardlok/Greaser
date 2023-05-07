// Async Global Multi Indexer
// Ex of needs
// fn adduser() { let (user_id, workspace_id) = Indexer.get_next_for(user, workspace)}

pub mod Nestindex {
    use super::SerialInt::*;
    use hashbrown::HashMap;

    // Indexing with two layers, a parent layer and a child layer.
    pub struct Nestindex {
        data: HashMap<u8, SerialGenerator>,
        tally: SerialGenerator<u8>,
    }
    impl Nestindex {
        pub fn new() -> Nestindex {
            Nestindex {
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
        use crate::TypeTally::Tally;

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
pub mod CraftingTable {
    use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};

    use super::*;
    use hashbrown::HashMap;

    pub struct Table {
        //root// Runtimes   //Tasks       // Components
        root: HashMap<Noid, HashMap<Noid, Vec<EdgeType>>>,
        scopes: HashMap<Scope, Vec<Noid>>,
    }

    // Real time information on the build
    pub struct Stats {
        runtimes: usize,
        tasks: usize,
        edges: usize,
        scopes: usize,
    }

    impl Table {
        //////////////////////////////////////////////
        // Inputs
        /////////
        // Create a new Table with at least one Runtime/Task instance. In
        // any use case there should be at least one of these. However, this
        // is not limited to just working with one. Multiple Nodes, Edges, and
        // Scopes should be configurable simultaneously.
        pub fn new(noids: Vec<Noid>) -> Table {
            let mut root = HashMap::new();
            let mut scopes = HashMap::new();
            for noid in noids {
                root.entry(noid.0).or_insert(HashMap::new());
                scopes.entry(noid).or_insert(vec![noid]);
            }
            Table { root, scopes }
        }
        // Add a new runtime to the roster, this also addes the additional
        // node entry plus scope entry respectively.
        pub fn add_rts(self, noids: Vec<Noid>) -> Table {
            for noid in noids {
                self.root.insert(noid.0, HashMap::new());
                self.scopes.entry(noid).or_insert(vec![noid]);
            }
        }
        // Add new tasks to the roster
        pub fn add_tasks(self, noids: Vec<Noid>) -> Table {
            for noid in noids {
                self.root
                    .entry(noid.0)
                    .and_modify(|rt| rt.entry(noid.1).or_insert(Vec::new()));
                self.scopes.entry(noid).or_insert(vec![noid]);
            }
        }
        // Add Edge components to the node configuration
        pub fn add_edges(self, noids: Vec<Noid>, edges: Vec<EdgeType>) -> Table {
            for noid in noids {
                self.root
                    .entry(noid.0)
                    .and_modify(|rt| rt.entry(noid.1).or_insert(edges))
            }
        }
        // Add Scope designation to the nod configuration
        pub fn add_scopes(self, scope: Noid, noids: Vec<Noid>) -> Table {
            for noid in noids {
                self.scopes.entry(scope).and_modify(|scope| {
                    scope
                        .entry(noid)
                        .and_modify(|group| group.append(noid))
                        .or_insert()
                })
            }
        }
        //
        //////////////////////////////////////////////
        // Outputs
        //////////
        pub fn stats(self) -> Stats {
            let mut count_r = 0usize; // Runtime count
            let mut count_t = 0usize; // Task count
            let mut count_e = 0usize; // Edge count
            let mut count_s = 0usize; // Scope count
            self.root.drain().inspect(|(rt, noids)| {
                count_r += 1;
                count_t += noids.len();
                noids.drain().inspect(|_, edges| count_e += edges.len());
            });
            count_s += self.scopes.len();
            Stats {
                runtimes: count_r,
                tasks: count_t,
                edges: count_e,
                scopes: count_s,
            }
        }
        pub fn runtimes(self) -> Vec<u8> {
            self.root.keys().collect()
        }
        pub fn nodes(self) -> Vec<Noid> {
            self.root.values().into_iter().sort().dedup()
        }
        pub fn scopes(self) -> Vec<Scope> {
            self.scopes.keys().collect()
        }
    }
}
