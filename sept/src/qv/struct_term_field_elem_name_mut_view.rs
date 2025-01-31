use crate::{dy, qv, Result};

#[derive(Debug)]
pub struct StructTermFieldElemNameMutView<'a> {
    pub struct_term: &'a mut dy::StructTerm,
    pub field_index: usize,
}

impl<'a> StructTermFieldElemNameMutView<'a> {
    pub fn new(struct_term: &'a mut dy::StructTerm, field_index: usize) -> Result<Self> {
        anyhow::ensure!(
            field_index < struct_term.len(),
            "StructTermFieldElemNameMutView field_index out of bounds"
        );
        Ok(Self {
            struct_term,
            field_index,
        })
    }
}

impl<'a> qv::ApplyEditT for StructTermFieldElemNameMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        // Apply the edit to a clone of the field name, then check if it was a valid edit.
        let original_field_name = self.struct_term.get_field_name(self.field_index)?;
        let mut field_name_clone = original_field_name.clone();
        field_name_clone.apply_edit(edit)?;
        if field_name_clone.as_str() == original_field_name.as_str() {
            // No change was made, so nothing needs to be done.
            return Ok(());
        }

        // Note that the call set_field_name will not alter self.struct_term if it returns error.
        self.struct_term
            .set_field_name(self.field_index, field_name_clone)?;
        Ok(())
    }
}

impl<'b> qv::QueryMutAndApplyEditT for StructTermFieldElemNameMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // Pass the edit on to a clone of the field_name, and then later reconcile this with the
        // struct_term.  This is done because there could be an error during the edit, and
        // we want the operation to be atomic.
        let new_field_name = {
            // This unwrap shouldn't fail because of the check in new().
            let existing_field_name = self.struct_term.get_field_name(self.field_index).unwrap();
            let mut new_field_name = existing_field_name.to_string();

            // Re-borrow the address iterator items with a shorter lifetime.
            let mut address_token_i = address_token_i.map(|x| &*x);
            new_field_name.query_mut_and_apply_edit(&mut address_token_i, edit)?;
            // If the field_name wasn't changed, then there's no need to do anything.
            if new_field_name == *existing_field_name {
                return Ok(());
            }
            new_field_name
        };
        self.struct_term
            .set_field_name(self.field_index, new_field_name)?;
        Ok(())
    }
}

// SingleQueryMutT would be possible if there was an inner and outer query, and the outer query
// runs constraint checks after the inner query has apply_edit run on it.
// impl<'b> qv::SingleQueryMutT<dy::Value> for StructTermFieldElemNameMutView<'b> {
//     type ReturnType<'a> = StructTermFieldElemNameMutViewQuery<'a> where 'b: 'a;
//     type Error = Error;
//     fn run_single_query_mut<'a>(
//         &'a mut self,
//         address_token: &dy::Value,
//     ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
//         if let Some(sub_index) = address_token.downcast_ref::<u32>() {
//             match *sub_index {
//                 0 => {
//                     // TODO: This has to use qv::StructTermFieldElemNameMutView

//                     // unimplemented!("todo");
//                     Ok(
//                         qv::StructTermFieldElemNameMutView::new(
//                             self.struct_term,
//                             self.field_index,
//                         )?
//                         .into(),
//                     )
//                 }
//                 1 => Ok(
//                     StructTermFieldElemNameMutViewQuery::StructTermFieldElemTypeMutView(
//                         self.struct_term.get_field_type_mut(self.field_index)?,
//                     ),
//                 ),
//                 _ => {
//                     anyhow::bail!("StructTermFieldElemNameMutView::run_single_query_mut; invalid field_index value -- must be 0 (field name) or 1 (field type)");
//                 }
//             }
//         } else {
//             // TODO: Support field names as addresses.
//             use st::StringifiableT;
//             anyhow::bail!(
//                 "StructTermFieldElemNameMutView::run_single_query_mut; unrecognized address_token {}",
//                 address_token.stringify()
//             );
//         }
//     }
// }
