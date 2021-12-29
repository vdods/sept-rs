use crate::st::TermTrait;

pub trait Inhabits<Rhs: TermTrait> {
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
