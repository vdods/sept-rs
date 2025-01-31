use crate::{dy, qv, Result};

pub trait QueryMutAndApplyEditT {
    // TODO: Rename to something better, this is ridiculous.
    // TODO: Maybe 's is not needed and can be elided.
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a;
}

impl<T> QueryMutAndApplyEditT for T
where
    T: qv::SingleQueryMutT<dy::Value> + qv::ApplyEditT,
    anyhow::Error: From<<T as qv::SingleQueryMutT<dy::Value>>::Error>,
{
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        if let Some(address_token) = address_token_i.next() {
            // Re-borrow address_token with a shorter lifetime.
            let address_token = &*address_token;
            // Re-borrow the iterator items with a shorter lifetime.
            let mut address_token_i = address_token_i.map(|x| &*x);
            self.run_single_query_mut(address_token)?
                .query_mut_and_apply_edit(&mut address_token_i, edit)
        } else {
            self.apply_edit(edit)
        }
    }
}
