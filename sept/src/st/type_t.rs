use crate::st::TermT;

/// This is a marker trait that indicates that the term is a type, and therefore
/// it's possible that terms can inhabit it.  The specific definition of inhabitation
/// is still up to an impl of the trait InhabitsT<T>.
pub trait TypeT: TermT {}
