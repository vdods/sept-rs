use crate::{dy, st, Result};

pub trait Queryable: st::TermTrait {
    // Not sure if the return type is right.
    // TODO: Make this take an iterator for the address tokens.
    fn query<'a>(&'a self, address_v: &[dy::Value]) -> Result<&'a dy::ValueGuts>;
    fn query_mut<'a>(&'a mut self, address_v: &[dy::Value]) -> Result<&'a mut dy::ValueGuts>;
}
