use crate::{dy, st, Result};

#[derive(Debug)]
pub struct OrderedMapTermMutView<'a>(&'a mut dy::OrderedMapTerm);

impl<'a> OrderedMapTermMutView<'a> {
    pub fn new(ordered_map_term: &'a mut dy::OrderedMapTerm) -> Box<Self> {
        Box::new(Self(ordered_map_term))
    }
}

impl<'b> dy::QueryMutTrait for OrderedMapTermMutView<'b> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        if let Some(address_char) = first_address.downcast_ref::<char>().map(|c| *c) {
            match address_char {
                // keys
                'k' => {
                    anyhow::ensure!(address_i.peek().is_some(), "OrderedMapTerm query address 'k' requires a second address (the key being addressed) but none was provided");
                    let second_address = address_i.next().unwrap();
                    // Pass on the rest of the address to the key view's impl of query.
                    dy::OrderedMapTermKeyMutView::new((*self).0, second_address)?
                        .run_query_mut(&mut address_i)
                }
                // values
                'v' => {
                    anyhow::ensure!(address_i.peek().is_some(), "OrderedMapTerm query address 'v' requires a second address (the key of the value being addressed) but none was provided");
                    let second_address = address_i.next().unwrap();
                    anyhow::ensure!(self.0.contains_key(second_address), "OrderedMapTerm query address 'v' was followed by a a value (the key of the value being addressed) that was not found in the OrderedMapTerm");
                    // Pass on the rest of the address to the val.
                    dy::ValueMutView::new(self.0.get_mut(second_address).unwrap())
                        .run_query_mut(&mut address_i)
                }
                // key/value pairs
                'p' => {
                    unimplemented!("not yet");
                }
                _ => {
                    use st::Stringifiable;
                    anyhow::bail!(
                        "OrderedMapTerm query doesn't support address: {}",
                        first_address.stringify()
                    )
                }
            }
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "OrderedMapTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> dy::Editable for OrderedMapTermMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // TEMP HACK -- this is rather silly, but is a quick way to get the right behavior for now.
        use dy::QueryMutTrait;
        // Re-borrow the address iterator items with a shorter lifetime.
        // let mut address_i = address_i.map(|x| &*x);
        // Note that this can't be Self, because this introduces a new, shorter lifetime.
        OrderedMapTermMutView::new(self.0)
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for OrderedMapTermMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}

impl<'b> dy::QueryMutViewTrait for OrderedMapTermMutView<'b> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<st::NoOp>() {
            // Nothing to do.
        } else if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<dy::OrderedMapTerm>(),
                "Utf8OrderedMapTermTerm ReplacementTerm edit expected old_data to be OrderedMapTerm"
            );
            anyhow::ensure!(
                edit.new_data.is::<dy::OrderedMapTerm>(),
                "Utf8OrderedMapTermTerm ReplacementTerm edit expected new_data to be OrderedMapTerm"
            );
            let old_ordered_map_term = edit.old_data.downcast_into::<dy::OrderedMapTerm>();
            let new_ordered_map_term = edit.new_data.downcast_into::<dy::OrderedMapTerm>();
            anyhow::ensure!(*self.0 == old_ordered_map_term, "Utf8OrderedMapTermTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_ordered_map_term);
            *self.0 = new_ordered_map_term;
        } else {
            anyhow::bail!("OrderedMapTerm does not support edit: {}", edit);
        }
        Ok(())
    }
}
