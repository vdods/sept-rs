pub trait Inhabits<Rhs> {
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
