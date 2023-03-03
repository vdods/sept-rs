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

/// This is a bit of a hack, but is a practical way to iterate over the tuple of non-parametric
/// terms in a strongly typed way, without resorting to dy::Value.
#[macro_export]
macro_rules! for_each_parametric_term {
    ($T:ident, $e:expr) => {
        (
            { type T = $crate::dy::ArrayTerm;                  $e },
            { type T = $crate::dy::GlobalSymRefTerm;                  $e },
//             { type T = $crate::dy::LocalSymRefTerm;                  $e },
            { type T = $crate::dy::StructTerm;                  $e },
            { type T = $crate::dy::StructTermTerm;                  $e },
            { type T = $crate::dy::TupleTerm;                  $e },
            // TODO: Should this just be bool?  BoolTerm is a type alias for bool.
            { type T = $crate::st::BoolTerm;                  $e },
            { type T = $crate::st::Float32Term;                  $e },
            { type T = $crate::st::Float64Term;                  $e },
            { type T = $crate::st::Sint8Term;                  $e },
            { type T = $crate::st::Sint16Term;                  $e },
            { type T = $crate::st::Sint32Term;                  $e },
            { type T = $crate::st::Sint64Term;                  $e },
            { type T = $crate::st::Uint8Term;                  $e },
            { type T = $crate::st::Uint16Term;                  $e },
            { type T = $crate::st::Uint32Term;                  $e },
            { type T = $crate::st::Uint64Term;                  $e },
            { type T = $crate::st::Utf8StringTerm;                  $e },
        )
    };
}
