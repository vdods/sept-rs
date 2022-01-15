// #![feature(iter_zip)]
// #![feature(adt_const_params)] -- TODO: Would be nice to use this, so that enums can be used as const generic params

mod dag;
/// Contains the sept runtime and dynamic types.
pub mod dy;
mod poset;
/// Contains static types and traits.
pub mod st;

pub use anyhow::{Error, Result};
pub use crate::{
    dag::{DirectedAcyclicGraph, EdgeSetMap, IncludeNode, NodeSet},
    poset::{PartialOrder, PartiallyOrderedSet},
};
