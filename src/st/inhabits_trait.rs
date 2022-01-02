use crate::st;

// TODO: Rename this to ConditionallyInhabits (or maybe put this in `mod dy`)
// and then create UnconditionallyInhabits which doesn't need a `&self` param (or maybe put it in `mod st`)
pub trait Inhabits<Rhs: st::TypeTrait + 'static>: st::TermTrait + 'static {
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
