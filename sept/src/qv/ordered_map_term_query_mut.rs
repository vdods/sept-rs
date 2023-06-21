use crate::{dy, qv, Result};

#[derive(Debug, derive_more::From)]
pub enum OrderedMapTermQueryMut<'a> {
    OrderedMapTermKeyMutView(qv::OrderedMapTermKeyMutView<'a>),
    OrderedMapTermValMutView(qv::OrderedMapTermValMutView<'a>),
}

// TODO: Derive this
impl<'b> qv::QueryMutAndApplyEditTrait for OrderedMapTermQueryMut<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::OrderedMapTermKeyMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
            Self::OrderedMapTermValMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
        }
    }
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditTrait for OrderedMapTermQueryMut<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::OrderedMapTermKeyMutView(v) => v.apply_edit(edit),
            Self::OrderedMapTermValMutView(v) => v.apply_edit(edit),
        }
    }
}
