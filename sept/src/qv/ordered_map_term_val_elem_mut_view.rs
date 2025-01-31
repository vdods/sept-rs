use crate::{dy, qv, Result};

// NOTE: This doesn't need to exist, it can simply pass to the value.
#[derive(Debug)]
pub struct OrderedMapTermValElemMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object.
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
    // TODO: Attempt to make this a reference again (have to modify SingleQueryMutT::run_single_query_mut
    // and that's likely to be a big can of worms)
    pub key: dy::Value,
}

impl<'a> OrderedMapTermValElemMutView<'a> {
    pub fn new(ordered_map_term: &'a mut dy::OrderedMapTerm, key: dy::Value) -> Result<Self> {
        anyhow::ensure!(
            ordered_map_term.contains_key(&key),
            "OrderedMapTerm doesn't contain the specified key"
        );
        Ok(Self {
            ordered_map_term,
            key,
        })
    }
}

impl<'a> qv::ApplyEditT for OrderedMapTermValElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // Just forward to the val itself.  The unwrap can't fail because it was checked in the constructor.
        self.ordered_map_term
            .get_mut(&self.key)
            .unwrap()
            .apply_edit(edit)
    }
}

impl<'b> qv::QueryMutAndApplyEditT for OrderedMapTermValElemMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // Just forward to the val itself.  The unwrap can't fail because it was checked in the constructor.
        self.ordered_map_term
            .get_mut(&self.key)
            .unwrap()
            .query_mut_and_apply_edit(address_token_i, edit)
    }
}
