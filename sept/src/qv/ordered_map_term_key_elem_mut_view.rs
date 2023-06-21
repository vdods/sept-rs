use crate::{dy, qv, Result};

#[derive(Debug)]
pub struct OrderedMapTermKeyElemMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an OrderedMapTerm mut view object.
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
    // TODO: Attempt to make this a reference again (have to modify SingleQueryMut::run_single_query_mut
    // and that's likely to be a big can of worms)
    pub key: dy::Value,
}

impl<'a> OrderedMapTermKeyElemMutView<'a> {
    pub fn new(ordered_map_term: &'a mut dy::OrderedMapTerm, key: dy::Value) -> Result<Self> {
        // NOTE: This check isn't right, in order to support insertion, key might not be contained.
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

impl<'a> qv::ApplyEditTrait for OrderedMapTermKeyElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // Edit a clone of the key, since there could be an error during the call to apply_edit, and
        // we want the operation to be atomic.
        let mut key_clone = self.key.clone();
        key_clone.apply_edit(edit)?;
        // If the key wasn't changed, then there's no need to do anything.
        if key_clone == self.key {
            return Ok(());
        }
        // Otherwise check for collision with an existing key
        anyhow::ensure!(!self.ordered_map_term.contains_key(&key_clone), "Can't apply edit key {} -> {} of OrderedMapTerm because the edited key value is already present", self.key, key_clone);
        // Remove the key-value pair having the old key, then add new one.
        let mapped_value = self.ordered_map_term.remove(&self.key).unwrap();
        self.ordered_map_term.insert(key_clone, mapped_value);
        Ok(())
    }
}

impl<'b> qv::QueryMutAndApplyEditTrait for OrderedMapTermKeyElemMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // Pass the edit on to a clone of the key, and then later reconcile this with the
        // ordered_map_term.  This is done because there could be an error during the edit, and
        // we want the operation to be atomic.
        let mut key_clone = self.key.clone();
        // Re-borrow the address iterator items with a shorter lifetime.
        let mut address_token_i = address_token_i.map(|x| &*x);
        // This requires Runtime support.
        key_clone.query_mut_and_apply_edit(&mut address_token_i, edit)?;
        // If the key wasn't changed, then there's no need to do anything.
        if key_clone == self.key {
            return Ok(());
        }
        // Otherwise check for collision with an existing key
        anyhow::ensure!(!self.ordered_map_term.contains_key(&key_clone), "Can't apply edit key {} -> {} of OrderedMapTerm because the edited key value is already present", self.key, key_clone);
        // Remove the key-value pair having the old key, then add new one.
        let mapped_value = self.ordered_map_term.remove(&self.key).unwrap();
        self.ordered_map_term.insert(key_clone, mapped_value);
        Ok(())
    }
}
