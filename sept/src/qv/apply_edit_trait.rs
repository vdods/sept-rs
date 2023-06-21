use crate::{dy, qv, st, Result};

// TODO: Figure out if EvalTrait supertrait is really needed.  Probably not, unless there's some definitional
// reason why it's needed for QueryMutViewTrait.  Actually, a given mutation might alter a EvalTrait
// such that it's no longer valid, so maybe EvalTrait is not even well-defined here.
// pub trait QueryMutViewTrait: qv::EvalTrait {
pub trait ApplyEditTrait {
    /// This operation must be atomic -- if an error is to be returned, it must not alter the queried data,
    /// and if the queried data is altered, then this method call must succeed.
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()>;
}

pub fn generic_apply_edit<T: std::any::Any + PartialEq>(x: &mut T, edit: dy::Value) -> Result<()> {
    if edit.is::<st::NoOp>() {
        // Nothing to do.
        Ok(())
    } else if edit.is::<qv::ReplacementTerm>() {
        let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
        anyhow::ensure!(
            replacement_term.old_data.is::<T>(),
            "ReplacementTerm edit expected old_data to have identical type",
        );
        anyhow::ensure!(
            replacement_term.new_data.is::<T>(),
            "ReplacementTerm edit expected new_data to have identical type",
        );
        let old_value = replacement_term.old_data.downcast_into::<T>();
        let new_value = replacement_term.new_data.downcast_into::<T>();
        anyhow::ensure!(
            *x == old_value,
            "ReplacementTerm edit expected current value to match old_data"
        );
        *x = new_value;
        Ok(())
    } else {
        anyhow::bail!("Unsupported edit {:?}", edit);
    }
}
