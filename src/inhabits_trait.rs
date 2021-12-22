pub trait Inhabits<Rhs> {
//     type Rhs = Rhs;
    fn inhabits(&self, rhs: &Rhs) -> bool;
}
