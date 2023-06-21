use crate::qv;

#[derive(Clone, Debug)]
pub struct EmptyQuery;

impl qv::ApplyEditTrait for EmptyQuery {
    fn apply_edit(&mut self, _edit: crate::dy::Value) -> anyhow::Result<()> {
        anyhow::bail!("EmptyQuery does not support edits");
    }
}

impl qv::QueryMutAndApplyEditTrait for EmptyQuery {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        _address_token_i: &mut dyn std::iter::Iterator<Item = &'a crate::dy::Value>,
        _edit: crate::dy::Value,
    ) -> anyhow::Result<()>
    where
        's: 'a,
    {
        anyhow::bail!("EmptyQuery does not support query or edits");
    }
}
