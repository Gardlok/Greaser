use crate::*;
use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct DataStruct<P, C>(
    // P: Priority C: Class
    u16,
    PhantomData<(P, C)>,
);

pub type Data = DataStruct<(), ()>;
fn trythis() {
    //let a = Node::Generate(10);
}
