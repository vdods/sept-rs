use crate::{dy, st, Result};

/// Defines a particular type of diff that can be applied to a particular type of term.
// TODO: Figure out if the use of dy warrants putting this in the dy module.
pub trait Diffable<D: st::DiffTrait>: st::TermTrait {
    /// Apply a diff to the term at the given address.
    fn apply_diff<'a>(
        &mut self,
        address_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        diff: &D,
    ) -> Result<()>;
}

// TODO: Move these to an appropriate place.

impl<T: st::TermTrait> Diffable<st::NoOp> for T {
    fn apply_diff<'a>(
        &mut self,
        mut address_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        _diff: &st::NoOp,
    ) -> Result<()> {
        anyhow::ensure!(
            address_i.next().is_none(),
            "NoOp only supports apply_diff for terminal addresses"
        );
        Ok(())
    }
}

impl Diffable<dy::InsertionTerm> for st::Utf8StringTerm {
    fn apply_diff<'a>(
        &mut self,
        mut address_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        diff: &dy::InsertionTerm,
    ) -> Result<()> {
        let index_value_guts = address_i.next().ok_or_else(|| {
            anyhow::anyhow!("Utf8StringTerm::apply_diff with InsertionTerm requires its address to be an index and nothing following it")
        })?;
        anyhow::ensure!(
            address_i.next().is_none(),
            "Utf8StringTerm::apply_diff with InsertionTerm requires its address to be an index and nothing following it"
        );
        // TEMP HACK: Assume that the index is u32.
        let index = *index_value_guts.downcast_ref::<u32>().ok_or_else(|| {
            anyhow::anyhow!(
                "Utf8StringTerm::apply_diff with InsertionTerm expected u32 as index, but got {:?}",
                index_value_guts
            )
        })?;
        let char = *diff.new_data.downcast_ref::<char>().ok_or_else(|| {
            anyhow::anyhow!(
                "Utf8StringTerm::apply_diff with InsertionTerm expected UnicodeCharTerm for data"
            )
        })?;
        self.insert(index as usize, char);
        Ok(())
    }
}

impl Diffable<dy::DeletionTerm> for st::Utf8StringTerm {
    fn apply_diff<'a>(
        &mut self,
        mut address_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        diff: &dy::DeletionTerm,
    ) -> Result<()> {
        let index_value_guts = address_i.next().ok_or_else(|| {
            anyhow::anyhow!("Utf8StringTerm::apply_diff with DeletionTerm requires its address to be an index and nothing following it")
        })?;
        anyhow::ensure!(
            address_i.next().is_none(),
            "Utf8StringTerm::apply_diff with DeletionTerm requires its address to be an index and nothing following it"
        );
        // TEMP HACK: Assume that the index is u32.
        let index = *index_value_guts.downcast_ref::<u32>().ok_or_else(|| {
            anyhow::anyhow!(
                "Utf8StringTerm::apply_diff with DeletionTerm expected u32 as index, but got {:?}",
                index_value_guts
            )
        })?;
        let char = *diff.old_data.downcast_ref::<char>().ok_or_else(|| {
            anyhow::anyhow!(
                "Utf8StringTerm::apply_diff with DeletionTerm expected UnicodeCharTerm for data"
            )
        })?;
        anyhow::ensure!(self.chars().nth(index as usize) == Some(char), "Utf8StringTerm::apply_diff with DeletionTerm encountered unexpected character at given index");
        self.remove(index as usize);
        Ok(())
    }
}

// impl Diffable<dy::ReplacementTerm> for st::Utf8StringTerm {
//     fn apply_diff<'a>(
//         &mut self,
//         mut address_i: impl std::iter::Iterator<Item = &'a dy::Value>,
//         diff: &dy::ReplacementTerm,
//     ) -> Result<()> {
//         let next = address_i.next();
//         let next_next = address_i.next();
//         match (next, next_next) {
//             (Some(token0), Some(token1)) => {
//                 anyhow::bail!("Utf8StringTerm::apply_diff with ReplacementTerm requires its address to be an index and nothing following it")
//             }
//             (Some(_), None) => {}
//             (None, None) => {
//                 anyhow::bail!("Utf8StringTerm::apply_diff with ReplacementTerm requires its address to be an index and nothing following it")
//             }
//             (None, Some(_)) => {
//                 anyhow::bail!("Utf8StringTerm::apply_diff with ReplacementTerm requires its address to be an index and nothing following it")
//             }
//         }
//         let index_value_guts = address_i.next().ok_or_else(|| {
//             anyhow::anyhow!("Utf8StringTerm::apply_diff with ReplacementTerm requires its address to be an index and nothing following it")
//         })?;
//         anyhow::ensure!(
//             address_i.next().is_none(),
//             "Utf8StringTerm::apply_diff with InsertionTerm requires its address to be an index and nothing following it"
//         );
//         // TEMP HACK: Assume that the index is u32.
//         let index = *index_value_guts.downcast_ref::<u32>().ok_or_else(|| {
//             anyhow::anyhow!(
//                 "Utf8StringTerm::apply_diff with InsertionTerm expected u32 as index, but got {:?}",
//                 index_value_guts
//             )
//         })?;
//         let char = *diff.data.downcast_ref::<char>().ok_or_else(|| {
//             anyhow::anyhow!(
//                 "Utf8StringTerm::apply_diff with InsertionTerm expected UnicodeCharTerm for data"
//             )
//         })?;
//         self.insert(index as usize, char);
//         Ok(())
//     }
// }
