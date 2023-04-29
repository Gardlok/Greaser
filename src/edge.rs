use std::marker::PhantomData;

pub type Edge = EdgeStruct<(), (), ()>;

#[derive(PartialEq, Eq)]
pub struct EdgeStruct<P, S, T>(
    // P: Pattern S: Scope T: Type //
    u64,
    PhantomData<(P, S, T)>,
);
