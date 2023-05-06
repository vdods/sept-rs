use crate::{dy, st, Result};

#[derive(Debug)]
pub struct GenericMutView<'a, T>(&'a mut T);

// NOTE: I don't understand why some of the trait bounds are needed in the impls of all but QueryMutViewTrait

impl<'a, T: std::any::Any + std::fmt::Debug + PartialEq> GenericMutView<'a, T> {
    pub fn new(x: &'a mut T) -> Box<Self> {
        Box::new(Self(x))
    }
}

impl<'b, T: std::any::Any + std::fmt::Debug + PartialEq + Send + Sync> dy::QueryMutTrait
    for GenericMutView<'b, T>
{
    // impl<'b, T: st::TermTrait> dy::QueryMutTrait for GenericMutView<'b, T> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            Ok(self)
        } else {
            anyhow::bail!("GenericMutView doesn't support query on any nonempty address");
        }
    }
}

impl<'b, T: std::any::Any + std::fmt::Debug + PartialEq + Send + Sync> dy::Editable
    for GenericMutView<'b, T>
{
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            if edit.is::<st::NoOp>() {
                // Nothing to do.
            } else if edit.is::<dy::ReplacementTerm>() {
                let edit = edit.downcast_into::<dy::ReplacementTerm>();
                log::trace!(
                    "GenericMutView::apply_edit; self: {:?}, edit: {:?}",
                    self,
                    edit
                );
                // use st::Stringifiable;
                anyhow::ensure!(
                    edit.old_data.is::<T>(),
                    "GenericMutView ReplacementTerm edit expected old_data to match the type of {}",
                    // self.stringify()
                    "blah"
                );
                anyhow::ensure!(
                    edit.new_data.is::<T>(),
                    "ArrayTerm ReplacementTerm edit expected new_data to match the type of {}",
                    // self.stringify()
                    "blah"
                );
                let old_value = edit.old_data.downcast_into::<T>();
                let new_value = edit.new_data.downcast_into::<T>();
                anyhow::ensure!(*self.0 == old_value, "ArrayTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_value);
                *self.0 = new_value;
            } else {
                anyhow::bail!("GenericMutView does not support edit {:?}", edit);
            }
        } else {
            anyhow::bail!("GenericMutView doesn't support query on any nonempty address");
        }
        Ok(())
    }
}

impl<'b, T: std::any::Any + std::fmt::Debug + PartialEq + Send + Sync> dy::QueryViewTrait
    for GenericMutView<'b, T>
{
    // impl<'b, T: st::TermTrait> dy::QueryViewTrait for GenericMutView<'b, T> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}

impl<'b, T: std::any::Any + std::fmt::Debug + PartialEq + Send + Sync> dy::QueryMutViewTrait
    for GenericMutView<'b, T>
{
    // impl<'b, T: st::Stringifiable + st::TermTrait> dy::QueryMutViewTrait for GenericMutView<'b, T> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            log::trace!(
                "GenericMutView::apply_edit; self: {:?}, edit: {:?}",
                self,
                edit
            );
            // use st::Stringifiable;
            anyhow::ensure!(
                edit.old_data.is::<T>(),
                "GenericMutView ReplacementTerm edit expected old_data to match the type of {}",
                // self.stringify()
                "blah"
            );
            anyhow::ensure!(
                edit.new_data.is::<T>(),
                "ArrayTerm ReplacementTerm edit expected new_data to match the type of {}",
                // self.stringify()
                "blah"
            );
            let old_value = edit.old_data.downcast_into::<T>();
            let new_value = edit.new_data.downcast_into::<T>();
            anyhow::ensure!(*self.0 == old_value, "ArrayTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_value);
            *self.0 = new_value;
        } else {
            anyhow::bail!("GenericMutView only supports ReplacementTerm edits")
        }
        Ok(())
    }
}
