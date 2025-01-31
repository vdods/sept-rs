use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct StructTermFieldElemMutView<'a> {
    pub struct_term: &'a mut dy::StructTerm,
    pub field_index: usize,
}

impl<'a> StructTermFieldElemMutView<'a> {
    pub fn new(struct_term: &'a mut dy::StructTerm, field_index: usize) -> Result<Self> {
        anyhow::ensure!(
            field_index <= struct_term.len(),
            "StructTermFieldElemMutView field_index out of bounds"
        );
        Ok(Self {
            struct_term,
            field_index,
        })
    }
}

impl<'a> qv::ApplyEditT for StructTermFieldElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch more efficiently (look up table as in Runtime?)
        if edit.type_id() == std::any::TypeId::of::<qv::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<qv::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.field_index <= self.struct_term.len(), "StructTermFieldElemMutView InsertionTerm edit had out-of-bounds field_index (field_index: {}, struct_term len: {})", self.field_index, self.struct_term.len());
            // Check that the insertion term is valid -- it should be a 2-tuple
            // consisting of a String and then an inhabitant of Type.
            anyhow::ensure!(
                insertion_term.new_data.is::<dy::TupleTerm>(),
                "StructTermFieldElemMutView InsertionTerm edit had non-TupleTerm new_data"
            );
            let mut new_data_t = insertion_term.new_data.downcast_into::<dy::TupleTerm>();
            anyhow::ensure!(new_data_t.len() == 2, "StructTermFieldElemMutView InsertionTerm edit had new_data TupleTerm with length != 2");
            let field_type = new_data_t.pop().unwrap();
            let field_name_value = new_data_t.pop().unwrap();
            anyhow::ensure!(
                field_name_value.is::<String>(),
                "StructTermFieldElemMutView InsertionTerm edit had non-String field_name_value"
            );
            let field_name = field_name_value.downcast_into::<String>();
            // use st::InhabitsT;
            // anyhow::ensure!(
            //     field_type.inhabits(&st::Type),
            //     "StructTermFieldElemMutView InsertionTerm edit had non-Type field_type"
            // );
            self.struct_term
                .insert_field(self.field_index, field_name, field_type)?;
        } else if edit.type_id() == std::any::TypeId::of::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.field_index < self.struct_term.len(), "StructTermFieldElemMutView DeletionTerm edit had out-of-bounds field_index (field_index: {}, struct_term len: {})", self.field_index, self.struct_term.len());
            // Check that the deletion term is valid -- it should be a 2-tuple
            // consisting of a String and then an inhabitant of Type.
            anyhow::ensure!(
                deletion_term.old_data.is::<dy::TupleTerm>(),
                "StructTermFieldElemMutView DeletionTerm edit had non-TupleTerm old_data"
            );
            let mut old_data_t = deletion_term.old_data.downcast_into::<dy::TupleTerm>();
            anyhow::ensure!(old_data_t.len() == 2, "StructTermFieldElemMutView DeletionTerm edit had old_data TupleTerm with length != 2");
            let field_type = old_data_t.pop().unwrap();
            let field_name_value = old_data_t.pop().unwrap();
            anyhow::ensure!(
                field_name_value.is::<String>(),
                "StructTermFieldElemMutView DeletionTerm edit had non-String field_name_value"
            );
            let field_name = field_name_value.downcast_into::<String>();
            // use st::InhabitsT;
            // anyhow::ensure!(
            //     field_type.inhabits(&st::Type),
            //     "StructTermFieldElemMutView DeletionTerm edit had non-Type field_type"
            // );
            {
                let existing_field = self.struct_term.get(self.field_index).unwrap();
                anyhow::ensure!(
                    existing_field.0 == field_name,
                    "StructTermFieldElemMutView DeletionTerm edit had field_name that didn't match existing field_name"
                );
                anyhow::ensure!(
                    existing_field.1 == field_type,
                    "StructTermFieldElemMutView DeletionTerm edit had field_type that didn't match existing field_type"
                );
            }
            self.struct_term.remove_field(self.field_index)?;
        } else if edit.type_id() == std::any::TypeId::of::<qv::ReplacementTerm>() {
            // let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            // // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            // anyhow::ensure!(self.field_index < self.struct_term.len(), "StructTermFieldElemMutView ReplacementTerm edit had out-of-bounds field_index (field_index: {}, struct_term len: {})", self.field_index, self.struct_term.len());
            unimplemented!("todo");
        } else {
            anyhow::bail!("StructTermFieldElemMutView doesn't support edit: {}", edit);
        }
        Ok(())
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for StructTermFieldElemMutView<'b> {
    type ReturnType<'a> = StructTermFieldElemMutViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(sub_index) = address_token.downcast_ref::<u32>() {
            match *sub_index {
                0 => {
                    // TODO: This has to use qv::StructTermFieldElemNameMutView

                    // unimplemented!("todo");
                    Ok(
                        qv::StructTermFieldElemNameMutView::new(
                            self.struct_term,
                            self.field_index,
                        )?
                        .into(),
                    )
                }
                1 => Ok(
                    StructTermFieldElemMutViewQuery::StructTermFieldElemTypeMutView(
                        self.struct_term.get_field_type_mut(self.field_index)?,
                    ),
                ),
                _ => {
                    anyhow::bail!("StructTermFieldElemMutView::run_single_query_mut; invalid elem_index value -- must be 0 (field name) or 1 (field type)");
                }
            }
        } else {
            // TODO: Support field names as addresses.
            use st::StringifiableT;
            anyhow::bail!(
                "StructTermFieldElemMutView::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum StructTermFieldElemMutViewQuery<'a> {
    StructTermFieldElemNameMutView(qv::StructTermFieldElemNameMutView<'a>),
    // TODO: This might actually need to be StructTermFieldTypeElemMutView<'a> in case the
    // StructTermFieldElemMutView needs to handle the changing of the field type.
    StructTermFieldElemTypeMutView(&'a mut dy::Value),
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditT for StructTermFieldElemMutViewQuery<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::StructTermFieldElemNameMutView(v) => v.apply_edit(edit),
            Self::StructTermFieldElemTypeMutView(v) => v.apply_edit(edit),
        }
    }
}

impl<'b> qv::QueryMutAndApplyEditT for StructTermFieldElemMutViewQuery<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::StructTermFieldElemNameMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            }
            Self::StructTermFieldElemTypeMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            }
        }
    }
}
