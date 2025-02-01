use crate::{dy, qv, Result};

#[derive(Debug, derive_more::From)]
pub enum TupleTermQueryMut<'a> {
    TupleTermElemMutView(qv::TupleTermElemMutView<'a>),
}

// TODO: Derive this
impl<'b> qv::QueryMutAndApplyEditT for TupleTermQueryMut<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::TupleTermElemMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
        }
    }
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditT for TupleTermQueryMut<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::TupleTermElemMutView(v) => v.apply_edit(edit),
        }
    }
}
