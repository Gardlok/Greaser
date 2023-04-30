// Async Global Multi Indexer
// Ex of needs
// fn adduser() { let (user_id, workspace_id) = Indexer.get_next_for(user, workspace)}

use std::any::{Any, TypeId};
use std::collections::HashMap;

// Basic (non async) Multi Indexer keyed by type
pub struct TypeTally {
    data: HashMap<TypeId, usize>,
}
impl TypeTally {
    pub fn new() -> TypeTally {
        TypeTally {
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
