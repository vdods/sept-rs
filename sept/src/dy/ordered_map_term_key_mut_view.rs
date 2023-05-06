use crate::{dy, Result};

#[derive(Debug)]
pub struct OrderedMapTermKeyMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an OrderedMapTerm mut view object.
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
    pub key: &'a dy::Value,
}

impl<'a> OrderedMapTermKeyMutView<'a> {
    pub fn new(
        ordered_map_term: &'a mut dy::OrderedMapTerm,
        key: &'a dy::Value,
    ) -> Result<Box<Self>> {
        anyhow::ensure!(
            ordered_map_term.contains_key(key),
            "OrderedMapTerm doesn't contain the specified key"
        );
        Ok(Box::new(Self {
            ordered_map_term,
            key,
        }))
    }
}

impl<'b> dy::QueryMutTrait for OrderedMapTermKeyMutView<'b> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            // Otherwise, pass it on to the key.
            // NOTE: This won't work for nested maps, it needs to be able to propagate backward.
            panic!("not supported yet");
            // TODO: Have to make some sort of wrapped thing that stores this QueryMutTrait object
            // so that it can propagate backward.
            // JAMMING/TEMP HACK
            // use dy::QueryableMutDynTrait;
            // let terminal_query_mut = self.key.make_and_run_query_mut(&mut address_i)?;
            // let mut wrapped = WrappedQueryMut::new(self, terminal_query_mut)

            // self.key.make_and_run_query_mut(&mut address_i)
        }
    }
}

impl<'b> dy::Editable for OrderedMapTermKeyMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
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
        let mut address_i = address_i.map(|x| &*x);
        // This requires Runtime support.
        key_clone.query_mut_and_apply_edit(&mut address_i, edit)?;
        // If the key wasn't changed, then there's no need to do anything.
        if key_clone == *self.key {
            return Ok(());
        }
        // Otherwise check for collision with an existing key
        anyhow::ensure!(!self.ordered_map_term.contains_key(&key_clone), "Can't apply edit key {} -> {} of OrderedMapTerm because the edited key value is already present", self.key, key_clone);
        // Remove the key-value pair having the old key, then add new one.
        let mapped_value = self.ordered_map_term.remove(self.key).unwrap();
        self.ordered_map_term.insert(key_clone, mapped_value);
        Ok(())
    }
}

impl<'a> dy::QueryViewTrait for OrderedMapTermKeyMutView<'a> {
    fn queried_value<'b>(&'b self) -> Result<dy::MaybeDereferencedValue<'b>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.key))
    }
}

impl<'a> dy::QueryMutViewTrait for OrderedMapTermKeyMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // Edit a clone of the key, since there could be an error during the call to apply_edit, and
        // we want the operation to be atomic.
        let mut key_clone = self.key.clone();
        // This requires Runtime support.
        dy::ValueMutView::new(&mut key_clone).apply_edit(edit)?;
        // If the key wasn't changed, then there's no need to do anything.
        if key_clone == *self.key {
            return Ok(());
        }
        // Otherwise check for collision with an existing key
        anyhow::ensure!(!self.ordered_map_term.contains_key(&key_clone), "Can't apply edit key {} -> {} of OrderedMapTerm because the edited key value is already present", self.key, key_clone);
        // Remove the key-value pair having the old key, then add new one.
        let mapped_value = self.ordered_map_term.remove(self.key).unwrap();
        self.ordered_map_term.insert(key_clone, mapped_value);
        Ok(())
    }
}
