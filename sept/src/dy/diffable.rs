use crate::{dy, st, Result};

// NOTE: This is the simplest possible interface for diffs, but may not be the most extensible,
// since once a type implements this trait, the kinds of diffs it can handle are fixed (due to
// the fact that it has to enumerate the kinds of supported diffs in code, which can't later be
// changed).
pub trait Diffable: st::TermTrait {
    /// Returns true iff the specified diff doesn't change the type of self, and therefore can
    /// be applied self in-place (via apply_diff_in_place).
    fn diff_is_mutation_in_place(&self, diff: &dy::ValueGuts) -> Result<bool>;
    /// This only works if the diff is a mutation of a given data type (i.e. the
    /// result of the diff isn't a different data type).
    fn apply_diff_in_place(&mut self, address_v: &[dy::Value], diff: &dy::ValueGuts) -> Result<()>;
    // /// This is the generic diff application method, consuming this Diffable and producing the result value.
    // fn apply_diff(self, diff: &dy::Value) -> Result<dy::Value>;
}

// TODO: Move these to a more appropriate place

impl Diffable for st::Utf8StringTerm {
    fn diff_is_mutation_in_place(&self, diff: &dy::ValueGuts) -> Result<bool> {
        let retval = if let Some(_) = diff.downcast_ref::<dy::NoOp>() {
            // Nothing to do.
            true
        } else if let Some(replacement_term) = diff.downcast_ref::<dy::ReplacementTerm>() {
            // Just need to check the type(s).
            replacement_term.old_data.is::<st::Utf8StringTerm>()
                && replacement_term.new_data.is::<st::Utf8StringTerm>()
        } else if let Some(element_insertion_term) = diff.downcast_ref::<dy::ElementInsertionTerm>()
        {
            // Just need to check the type(s).
            element_insertion_term.data.is::<st::Utf8StringTerm>()
        } else if let Some(element_deletion_term) = diff.downcast_ref::<dy::ElementDeletionTerm>() {
            // Just need to check the type(s).
            element_deletion_term.data.is::<st::Utf8StringTerm>()
        } else if let Some(element_replacement_term) =
            diff.downcast_ref::<dy::ElementReplacementTerm>()
        {
            // Just need to check the type(s).
            element_replacement_term.old_data.is::<st::Utf8StringTerm>()
                && element_replacement_term.new_data.is::<st::Utf8StringTerm>()
        } else {
            anyhow::bail!("Utf8StringTerm doesn't recognize {:?} as a diff", diff);
        };
        Ok(retval)
    }
    fn apply_diff_in_place(&mut self, address_v: &[dy::Value], diff: &dy::ValueGuts) -> Result<()> {
        anyhow::ensure!(
            address_v.is_empty(),
            "Utf8StringTerm only supports apply_diff_in_place for terminal addresses"
        );
        if let Some(_) = diff.downcast_ref::<dy::NoOp>() {
            // Nothing to do.
        } else if let Some(replacement_term) = diff.downcast_ref::<dy::ReplacementTerm>() {
            // Just need to check the type(s).
            let old_string = replacement_term
                .old_data
                .downcast_ref::<st::Utf8StringTerm>()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Utf8StringTerm encountered invalid old_data in ReplacementTerm"
                    )
                })?;
            let new_string = replacement_term
                .new_data
                .downcast_ref::<st::Utf8StringTerm>()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Utf8StringTerm encountered invalid new_data in ReplacementTerm"
                    )
                })?;
            anyhow::ensure!(
                self.as_str() == old_string.as_str(),
                "Utf8StringTerm did not match old_data in ReplacementTerm"
            );
            *self = new_string.clone().into();
        } else if let Some(_element_insertion_term) =
            diff.downcast_ref::<dy::ElementInsertionTerm>()
        {
            unimplemented!("blah");
        } else if let Some(_element_deletion_term) = diff.downcast_ref::<dy::ElementDeletionTerm>()
        {
            unimplemented!("blah");
        } else if let Some(_element_replacement_term) =
            diff.downcast_ref::<dy::ElementReplacementTerm>()
        {
            unimplemented!("blah");
        } else {
            anyhow::bail!("Utf8StringTerm doesn't recognize {:?} as a diff", diff);
        }
        Ok(())
    }
    // fn apply_diff(self, _diff: &dy::Value) -> Result<dy::Value> {
    //     unimplemented!("blah");
    // }
}
