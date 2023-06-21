use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Utf8StringTermLineView<'a> {
    pub string: &'a str,

    // cached values
    pub line_count: usize,
}

impl<'a> Utf8StringTermLineView<'a> {
    pub fn new(string: &'a str) -> Self {
        let line_count = st::split_inclusive_allow_trailing_empty(string, '\n').count();
        Self { string, line_count }
    }
    pub fn new_with_cached_line_count(string: &'a str, line_count: usize) -> Result<Self> {
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').count(),
            line_count,
            "programmer error: actual line_count does not match specified line_count"
        );
        Ok(Self { string, line_count })
    }
}

impl<'b> std::ops::Deref for Utf8StringTermLineView<'b> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.string
    }
}

impl<'b> qv::QueryTrait for Utf8StringTermLineView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            Box::new(qv::Utf8StringTermLineElemView::new(
                self.string,
                *element_index as usize,
            )?)
            .run_query(&mut address_token_i)
        } else {
            // TODO: Support "len" query.
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineView query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalTrait for Utf8StringTermLineView<'b> {
    /// This will produce an ArrayTerm populated with (clones of) the lines of the string,
    /// where each line includes the newline.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let line_v = st::split_inclusive_allow_trailing_empty(self.string, '\n')
            .map(|line| line.to_string().into_value())
            .collect::<Vec<dy::Value>>();
        let lines = dy::ArrayTerm::from(line_v);
        use dy::IntoValue;
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(lines.into_value()),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum Utf8StringTermLineViewQuery<'a> {
    Utf8StringTermLineElemView(qv::Utf8StringTermLineElemView<'a>),
}

impl<'b> qv::SingleQuery<dy::Value> for Utf8StringTermLineView<'b> {
    type ReturnType<'a> = Utf8StringTermLineViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(line_index) = address_token.downcast_ref::<u32>() {
            Ok(qv::Utf8StringTermLineElemView::new(self.string, *line_index as usize)?.into())
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineView::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}
