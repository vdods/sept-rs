use crate::st::TermTrait;

// TODO: Rename this to ConditionallyInhabits (or maybe put this in `mod dy`)
// and then create UnconditionallyInhabits which doesn't need a `&self` param (or maybe put it in `mod st`)
pub trait Inhabits<Rhs: TermTrait> {
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
