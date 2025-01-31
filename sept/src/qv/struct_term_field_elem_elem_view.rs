use crate::{dy, qv, Result};

#[derive(Clone, Debug)]
pub struct StructTermFieldElemElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    // This is the index of the field in the StructTerm.
    pub field_index: usize,
    // This is 0 (field name) or 1 (field type).
    pub sub_index: usize,
}

impl<'a> StructTermFieldElemElemView<'a> {
    pub fn new(
        struct_term: &'a dy::StructTerm,
        field_index: usize,
        sub_index: usize,
    ) -> Result<Self> {
        anyhow::ensure!(
            field_index <= struct_term.len(),
            "StructTermFieldElemElemView field_index out of bounds"
        );
        anyhow::ensure!(
            sub_index <= 1,
            "StructTermFieldElemElemView sub_index out of bounds (must be 0 (field name) or 1 (field type))"
        );
        Ok(Self {
            struct_term,
            field_index,
            sub_index,
        })
    }
    pub fn increment_field_index_by(&mut self, field_index_delta: isize) {
        self.field_index = self
            .field_index
            .saturating_add_signed(field_index_delta)
            .min(self.max_field_index());
    }
    pub fn go_home(&mut self) {
        self.sub_index = 0;
    }
    pub fn go_end(&mut self) {
        self.sub_index = self.max_sub_index();
    }
    pub fn increment_sub_index_by(&mut self, mut sub_index_delta: isize, allow_line_wrap: bool) {
        if allow_line_wrap {
            while sub_index_delta != 0 {
                if sub_index_delta < 0 {
                    // We're going backwards
                    if self.field_index == 0 {
                        // Since we're on the first field going backwards, this effectively uses up
                        // the whole delta.
                        self.sub_index = self.sub_index.saturating_add_signed(sub_index_delta);
                        sub_index_delta = 0;
                    } else {
                        let old_sub_index = self.sub_index;
                        self.sub_index = self.sub_index.saturating_add_signed(sub_index_delta);
                        let actual_sub_index_delta = -((old_sub_index - self.sub_index) as isize);
                        sub_index_delta -= actual_sub_index_delta;
                        if sub_index_delta < 0 {
                            // If there is still more delta to go, then we must wrap around to the
                            // the previous field.
                            self.increment_field_index_by(-1);
                            self.go_end();
                            // This uses up one char of the delta.
                            sub_index_delta += 1;
                        }
                    }
                } else if sub_index_delta > 0 {
                    // We're going forwards
                    if self.field_index == self.max_field_index() {
                        // Since we're on the last field going forwards, this effectively uses up
                        // the whole delta.
                        self.sub_index = self
                            .sub_index
                            .saturating_add_signed(sub_index_delta)
                            .min(self.max_sub_index());
                        sub_index_delta = 0;
                    } else {
                        let old_sub_index = self.sub_index;
                        self.sub_index = self
                            .sub_index
                            .saturating_add_signed(sub_index_delta)
                            .min(self.max_sub_index());
                        let actual_sub_index_delta = (self.sub_index - old_sub_index) as isize;
                        sub_index_delta -= actual_sub_index_delta;
                        if sub_index_delta > 0 {
                            // If there is still more delta to go, then we must wrap around to the
                            // the next field.
                            self.increment_field_index_by(1);
                            self.go_home();
                            // This uses up one char of the delta.
                            sub_index_delta -= 1;
                        }
                    }
                } else {
                    // No delta, do nothing.
                }
            }
        } else {
            self.sub_index = self
                .sub_index
                .saturating_add_signed(sub_index_delta)
                .min(self.max_sub_index())
        }
    }
    pub fn is_at_end_field_placeholder(&self) -> bool {
        self.field_index == self.max_field_index()
    }
    fn max_field_index(&self) -> usize {
        self.struct_term.len()
    }
    fn max_sub_index(&self) -> usize {
        1
    }
}

impl<'b> qv::EvalT for StructTermFieldElemElemView<'b> {
    /// This will produce either the field name or field value.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        assert!(self.sub_index <= 1);
        if self.sub_index == 0 {
            Ok(dy::MaybeDereferencedValue::make_ref(
                self.struct_term.get_field_name(self.field_index)?,
            ))
        } else {
            Ok(dy::MaybeDereferencedValue::make_ref(
                self.struct_term.get_field_type(self.field_index)?.as_ref(),
            ))
        }
    }
}

impl<'b> qv::QueryT for StructTermFieldElemElemView<'b> {
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
            // Forward to the field name or field type.
            let field_decl = self.struct_term.get(self.field_index).ok_or_else(|| {
                anyhow::anyhow!("StructTermFieldElemElemView field_index out of bounds")
            })?;
            assert!(self.sub_index <= 1);
            if self.sub_index == 0 {
                // Field name
                Box::new(qv::Utf8StringTermView::new(&field_decl.0)).run_query(&mut address_token_i)
            } else {
                // Field type
                Box::new(qv::ValueView::new(&field_decl.1)).run_query(&mut address_token_i)
            }
        }
    }
}

// impl<'b> qv::SingleQuery<dy::Value> for StructTermFieldElemElemView<'b> {
//     type ReturnType<'a> = StructTermFieldElemElemViewQuery<'a> where Self: 'a;
//     type Error = Error;
//     fn run_single_query<'a>(
//         &'a self,
//         address_token: &dy::Value,
//     ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
//         if let Some(elem_index) = address_token.downcast_ref::<u32>() {
//             match *elem_index {
//                 0 => Ok(StructTermFieldElemElemViewQuery::FieldName(
//                     self.struct_term.get_field_name(self.field_index)?,
//                 )),
//                 1 => Ok(StructTermFieldElemElemViewQuery::FieldType(
//                     self.struct_term.get_field_type(self.field_index)?,
//                 )),
//                 _ => {
//                     anyhow::bail!("StructTermFieldElemElemView::run_single_query; invalid elem_index value -- must be 0 (field name) or 1 (field type)");
//                 }
//             }
//         } else {
//             // TODO: Support field names as addresses.
//             use st::StringifiableT;
//             anyhow::bail!(
//                 "StructTermFieldElemElemView::run_single_query doesn't support address: {}",
//                 address_token.stringify()
//             );
//         }
//     }
// }

// pub enum StructTermFieldElemElemViewQuery<'a> {
//     FieldName(&'a String),
//     FieldType(&'a dy::Value),
// }
