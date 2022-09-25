// #![feature(iter_zip)]
// #![feature(adt_const_params)] -- TODO: Would be nice to use this, so that enums can be used as const generic params

mod dag;
/// Contains the sept runtime and dynamic types, most notably `dy::Value`, which is basically
/// `std::any::Any`, and the sept runtime.  One basic criteria for code belonging in the `dy`
/// module is if its core functioning depends on `dy::Value` or on the sept runtime.
pub mod dy;
pub mod parser;
mod poset;
pub mod scanner;
/// Contains static types and traits.  Theoretically everything here should function without use
/// of `dy::Value` or the sept runtime, although types in `st` typically also provide impls for
/// traits in `dy` so that they can be used in a `dy` way.
pub mod st;

pub use anyhow::{Error, Result};
pub use crate::{
    dag::{DirectedAcyclicGraph, EdgeSetMap, IncludeNode, NodeSet},
    poset::{PartialOrder, PartiallyOrderedSet},
};
