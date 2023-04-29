/////////////////////////////////////////////////////////////
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct AttriStruct(HashMap<TypeId, Box<dyn Any>>);

pub type Attri = AttriStruct;
impl Attri {
    fn insert<T: Any + 'static>(&mut self, t: T, fl: u8) {
        self.0.insert(TypeId::of::<T>(), Box::new(t));
    }
    fn get<T: Any + 'static>(&mut self) -> () {
        self.0.get(&TypeId::of::<T>());
    }
}
