use crate::{dy, st, Result};

#[derive(Debug)]
pub struct ValueMutView<'a>(&'a mut dy::Value);

impl<'a> ValueMutView<'a> {
    pub fn new(value: &'a mut dy::Value) -> Box<Self> {
        Box::new(Self(value))
    }
}

impl<'b> dy::QueryMutTrait for ValueMutView<'b> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a,
    {
        // Have to handle empty address case here, because there are edits that apply to Value-s
        // directly, and we need to be able to apply them to the ValueMutView.
        // NOTE: This may introduce some surprise when you do a query on ArrayTerm (whose elements
        // are Value-s which store specific concrete types such as u32) and you expected to get the
        // specific concrete element types (e.g. u32) back.
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            return Ok(self);
        }
        // Otherwise we can just delegate to the runtime.
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .query2_mut((*self).0.as_mut(), &mut address_i)
    }
}

impl<'b> dy::Editable for ValueMutView<'b> {
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
        ValueMutView::new(self.0)
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for ValueMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}

impl<'b> dy::QueryMutViewTrait for ValueMutView<'b> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        if edit.is::<st::NoOp>() {
            // Nothing needs to be done.
        } else if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            anyhow::ensure!(*self.0 == edit.old_data, "ValueMutView ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, edit.old_data);
            *self.0 = edit.new_data;
        } else {
            // dy::RUNTIME_LA
            //     .read()
            //     .unwrap()
            //     .apply_edit(self.0.as_mut(), edit)
            unimplemented!("blah");
        }
        Ok(())
    }
}
