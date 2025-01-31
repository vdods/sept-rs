use crate::st;

// TODO: Rename this to ConditionallyInhabits (or maybe put this in `mod dy`)
// and then create UnconditionallyInhabits which doesn't need a `&self` param (or maybe put it in `mod st`)
pub trait InhabitsT<Rhs: st::TypeT + 'static>: st::TermT + 'static {
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
