use crate::{dy, st, Result};

pub trait Queryable: st::TermTrait {
    // Not sure if the return type is right.
    // TODO: Figure out if it's feasible to use `address_v: &[&dy::ValueGuts]` so that the query
    // address tokens don't have to be on the heap (because dy::Value is a Box<dyn ValueGuts>).
    // Although then does that make using an actual &[dy::Value] difficult?
    // TODO: Actually, consider making this take an iterator for the address tokens.
    fn query<'a>(&'a self, address_v: &[dy::Value]) -> Result<&'a dy::ValueGuts>;
    fn query_mut<'a>(&'a mut self, address_v: &[dy::Value]) -> Result<&'a mut dy::ValueGuts>;
}
