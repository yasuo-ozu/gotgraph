pub mod graph;
pub mod vec_graph;

pub use graph::{basic::GraphBasic, Graph};
pub use vec_graph::VecGraph;

type Invariant<'a> = core::marker::PhantomData<fn(&'a ()) -> &'a ()>;
// #[allow(non_upper_case_globals)]
// const Invariant: Invariant<'_> = core::marker::PhantomData;
