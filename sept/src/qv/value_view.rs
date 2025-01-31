use crate::{dy, qv, Result};

#[derive(Clone, Debug)]
pub struct ValueView<'a>(&'a dy::Value);

impl<'a> ValueView<'a> {
    pub fn new(value: &'a dy::Value) -> Self {
        Self(value)
    }
}

impl<'b> qv::QueryT for ValueView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        Self: 'a,
    {
        // Have to handle empty address case here, because there are edits that apply to Value-s
        // directly, and we need to be able to apply them to the ValueView.
        // Have to handle empty address case here, because there are edits that apply to Value-s
        // directly, and we need to be able to apply them to the ValueMutView.
        // NOTE: This may introduce some surprise when you do a query on ArrayTerm (whose elements
        // are Value-s which store specific concrete types such as u32) and you expected to get the
        // specific concrete element types (e.g. u32) back.
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        // Otherwise we can just delegate to the runtime.
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .query((*self).0.as_ref(), &mut address_token_i)
    }
}

impl<'b> qv::EvalT for ValueView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0.as_ref()))
    }
}
