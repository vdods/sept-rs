use crate::{dy, qv, Result};

#[derive(Debug, derive_more::From)]
pub enum StructTermQueryMut<'a> {
    StructTermFieldElemMutView(qv::StructTermFieldElemMutView<'a>),
    // StructTermFieldNameMutView(qv::StructTermFieldNameMutView<'a>),
    // StructTermFieldTypeMutView(qv::StructTermFieldTypeMutView<'a>),
}

// TODO: Derive this
impl<'b> qv::QueryMutAndApplyEditT for StructTermQueryMut<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::StructTermFieldElemMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            } // Self::StructTermFieldNameMutView(v) => {
              //     v.query_mut_and_apply_edit(address_token_i, edit)
              // }
              // Self::StructTermFieldTypeMutView(v) => {
              //     v.query_mut_and_apply_edit(address_token_i, edit)
              // }
        }
    }
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditT for StructTermQueryMut<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::StructTermFieldElemMutView(v) => v.apply_edit(edit),
            // Self::StructTermFieldNameMutView(v) => v.apply_edit(edit),
            // Self::StructTermFieldTypeMutView(v) => v.apply_edit(edit),
        }
    }
}
