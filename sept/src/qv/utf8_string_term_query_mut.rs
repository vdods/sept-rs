use crate::{dy, qv, Result};

#[derive(Debug, derive_more::From)]
pub enum Utf8StringTermQueryMut<'a> {
    Utf8StringTermCharMutView(qv::Utf8StringTermCharMutView<'a>),
    Utf8StringTermLineMutView(qv::Utf8StringTermLineMutView<'a>),
}

// TODO: Derive this
impl<'b> qv::QueryMutAndApplyEditTrait for Utf8StringTermQueryMut<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::Utf8StringTermCharMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
            Self::Utf8StringTermLineMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
        }
    }
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditTrait for Utf8StringTermQueryMut<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::Utf8StringTermCharMutView(v) => v.apply_edit(edit),
            Self::Utf8StringTermLineMutView(v) => v.apply_edit(edit),
        }
    }
}
