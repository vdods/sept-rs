use crate::{dy, Result};

#[derive(Debug)]
pub struct Utf8StringTermMutView<'a>(&'a mut String);

impl<'a> Utf8StringTermMutView<'a> {
    pub fn new(string: &'a mut String) -> Box<Self> {
        Box::new(Self(string))
    }
}

impl<'b> dy::QueryMutTrait for Utf8StringTermMutView<'b> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        match first_address.downcast_ref::<String>().map(String::as_str) {
            Some("char") => {
                anyhow::ensure!(address_i.peek().is_some(), "Utf8StringTermView query address 'char' requires a second address (char index) but none was provided");
                let second_address = address_i.next().unwrap();
                anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTermView query address 'char' requires a second address (char index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                let char_index = *second_address.downcast_ref::<u32>().unwrap();
                // Pass on the rest of the address to the char view's impl of query_mut.
                dy::Utf8StringTermCharMutView::new(self.0, char_index as usize)?
                    .run_query_mut(&mut address_i)
            }
            Some("line") => {
                anyhow::ensure!(address_i.peek().is_some(), "Utf8StringTermView query address 'line' requires a second address (line index) but none was provided");
                let second_address = address_i.next().unwrap();
                anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTermView query address 'line' requires a second address (line index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                let line_index = *second_address.downcast_ref::<u32>().unwrap();
                // Pass on the rest of the address to the line view's impl of query_mut.
                dy::Utf8StringTermLineMutView::new(self.0, line_index as usize)?
                    .run_query_mut(&mut address_i)
            }
            _ => anyhow::bail!(
                "Utf8StringTermView query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            ),
        }
    }
}

impl<'b> dy::Editable for Utf8StringTermMutView<'b> {
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
        Utf8StringTermMutView::new(self.0)
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))

        // // This makes a copy, which is not ideal, but a non-static reference can't go in Value.
        // Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
        //     dy::Value::from(self.0.clone()).into(),
        // ))))
    }
}

impl<'a> dy::QueryMutViewTrait for Utf8StringTermMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<String>(),
                "Utf8StringTerm ReplacementTerm edit expected old_data to be String"
            );
            anyhow::ensure!(
                edit.new_data.is::<String>(),
                "Utf8StringTerm ReplacementTerm edit expected new_data to be String"
            );
            let old_string = edit.old_data.downcast_into::<String>();
            let new_string = edit.new_data.downcast_into::<String>();
            anyhow::ensure!(*self.0 == old_string, "Utf8StringTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_string);
            *self.0 = new_string;
        } else {
            anyhow::bail!("Utf8StringTerm does not support edit: {}", edit);
        }
        Ok(())
    }
}
