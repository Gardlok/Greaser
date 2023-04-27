pub type Edge = EdgeStruct<(), (), ()>;

#[derive(PartialEq, Eq)]
struct EdgeStruct<P, S, T>(
    // P: Pattern S: Scope T: Type //
    u64,
    PhantomData<(P, S, T)>,
);
