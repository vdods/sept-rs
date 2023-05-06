use crate::{dy, Result};

// NOTE: This doesn't need to exist, it can simply pass to the value.
#[derive(Debug)]
pub struct OrderedMapTermValMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object.
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
    pub key: &'a dy::Value,
}

impl<'a> OrderedMapTermValMutView<'a> {
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

impl<'b> dy::QueryMutTrait for OrderedMapTermValMutView<'b> {
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
            // Otherwise, pass it on to the value.  This unwrap can't fail because key containment was
            // checked in the constructor.
            let mut val = self.ordered_map_term.get_mut(self.key).unwrap();
            use dy::QueryableMutDynTrait;
            ValueMutView::new(&mut val).run_query_mut(&mut address_i)
        }
    }
}

impl<'b> dy::Editable for OrderedMapTermValMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        self.run_query_mut(address_i)?.apply_edit(edit)
    }
}

impl<'a> dy::QueryViewTrait for OrderedMapTermValMutView<'a> {
    fn queried_value<'b>(&'b self) -> Result<dy::MaybeDereferencedValue<'b>> {
        // This unwrap can't fail because key containment was checked in the constructor.
        Ok(dy::MaybeDereferencedValue::Ref(
            self.ordered_map_term.get(self.key).unwrap(),
        ))
    }
}

impl<'a> dy::QueryMutViewTrait for OrderedMapTermValMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        if edit.is::<st::NoOp>() {
            // Nothing to do.
        } else if edit.is::<dy::ReplacementTerm>() {
            // This unwrap can't fail because key containment was checked in the constructor.
            self.ordered_map_term.insert(self.key.clone(), edit.into());
        } else {
            anyhow::bail!("Can't apply edit {} to OrderedMapTermValMutView", edit);
        }
        // // Edit a clone of the key, since there could be an error during the call to apply_edit, and
        // // we want the operation to be atomic.
        // let mut key_clone = self.key.clone();
        // // This requires Runtime support.
        // dy::ValueMutView::new(&mut key_clone).apply_edit(edit)?;
        // // If the key wasn't changed, then there's no need to do anything.
        // // if dy::RUNTIME_LA
        // //     .read()
        // //     .unwrap()
        // //     .eq(self.key, key_clone.as_ref())
        // if key_clone == *self.key {
        //     return Ok(());
        // }
        // // Otherwise check for collision with an existing key
        // anyhow::ensure!(!self.ordered_map_term.contains_key(&key_clone), "Can't apply edit key {} -> {} of OrderedMapTerm because the edited key value is already present", self.key, key_clone);
        // // Remove old key, then add new one.
        // let mapped_value = self.ordered_map_term.remove(self.key).unwrap();
        // self.ordered_map_term.insert(key_clone, mapped_value);
        // Ok(())
    }
}
