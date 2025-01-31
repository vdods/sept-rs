use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct StructTermFieldElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    pub field_index: usize,
}

impl<'a> StructTermFieldElemView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm, field_index: usize) -> Result<Self> {
        anyhow::ensure!(
            field_index <= struct_term.len(),
            "StructTermFieldElemView field_index out of bounds"
        );
        Ok(Self {
            struct_term,
            field_index,
        })
    }
    pub fn new_from_field_name(struct_term: &'a dy::StructTerm, field_name: &str) -> Result<Self> {
        let elem_index = struct_term.index_of_named_field(field_name)?;
        Ok(Self {
            struct_term,
            field_index: elem_index,
        })
    }
    pub fn go_home(&mut self) {
        self.field_index = 0;
        // self.elem_o = self.struct_term.get(self.field_index);
    }
    pub fn go_end(&mut self) {
        self.field_index = self.max_field_index();
        assert!(self.struct_term.get(self.field_index) == None);
        // self.elem_o = None;
    }
    pub fn increment_field_index_by(&mut self, field_index_delta: isize) {
        self.field_index = self
            .field_index
            .saturating_add_signed(field_index_delta)
            .min(self.max_field_index());
        // self.elem_o = self.struct_term.get(self.field_index);
    }
    pub fn is_at_end_field_placeholder(&self) -> bool {
        self.field_index == self.max_field_index()
    }
    fn max_field_index(&self) -> usize {
        self.struct_term.len()
    }
}

impl<'b> qv::EvalT for StructTermFieldElemView<'b> {
    /// This will produce a TupleTerm (field_name, field_type), where each value is a clone.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let field_decl = self
            .struct_term
            .get(self.field_index)
            .ok_or_else(|| anyhow::anyhow!("StructTermFieldElemView field_index out of bounds"))?;
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(
                dy::TupleTerm::from(vec![
                    field_decl.0.clone().into(),
                    field_decl.1.clone().into(),
                ])
                .into(),
            ),
        )))
    }
}

impl<'b> qv::QueryT for StructTermFieldElemView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            let first_address = address_token_i.next().unwrap();
            // TODO: Figure out if SingleQuery can be used here to avoid code duplication.
            if let Some(sub_index) = first_address.downcast_ref::<u32>() {
                // match *elem_index {
                //     0 => {
                //         // Field name
                //         Box::new(qv::Utf8StringTermView::new(
                //             self.struct_term.get_field_name(self.elem_index)?,
                //         ))
                //         .run_query(&mut address_token_i)
                //     }
                //     1 => {
                //         // Field type
                //         Box::new(qv::ValueView::new(
                //             self.struct_term.get_field_type(self.elem_index)?,
                //         ))
                //         .run_query(&mut address_token_i)
                //     }
                //     _ => {
                //         anyhow::bail!("StructTermFieldElemView::run_query; invalid elem_index value (must be 0 or 1)");
                //     }
                // }
                Box::new(qv::StructTermFieldElemElemView::new(
                    self.struct_term,
                    self.field_index,
                    *sub_index as usize,
                )?)
                .run_query(&mut address_token_i)
            } else {
                use st::StringifiableT;
                anyhow::bail!(
                    "StructTermFieldElemView::run_query doesn't support address: {}",
                    first_address.stringify()
                );
            }
        }
    }
}

impl<'b> qv::SingleQueryT<dy::Value> for StructTermFieldElemView<'b> {
    type ReturnType<'a> = qv::StructTermFieldElemElemView<'a> where Self: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(sub_index) = address_token.downcast_ref::<u32>() {
            // match *elem_index {
            //     0 => Ok(StructTermFieldElemViewQuery::StructTermFieldElemNameView(
            //         self.struct_term.get_field_name(self.field_index)?,
            //     )),
            //     1 => Ok(StructTermFieldElemViewQuery::StructTermFieldElemTypeView(
            //         self.struct_term.get_field_type(self.field_index)?,
            //     )),
            //     _ => {
            //         anyhow::bail!("StructTermFieldElemView::run_single_query; invalid elem_index value -- must be 0 (field name) or 1 (field type)");
            //     }
            // }
            Ok(qv::StructTermFieldElemElemView::new(
                self.struct_term,
                self.field_index,
                *sub_index as usize,
            )?)
        } else {
            // TODO: Support field names as addresses.
            use st::StringifiableT;
            anyhow::bail!(
                "StructTermFieldElemView::run_single_query doesn't support address: {}",
                address_token.stringify()
            );
        }
    }
}

// pub enum StructTermFieldElemViewQuery<'a> {
//     StructTermFieldElemNameView(&'a String),
//     StructTermFieldElemTypeView(&'a dy::Value),
// }
