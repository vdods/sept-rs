use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

// TODO: This should just be a char view that takes a Utf8StringTermLineView, unless it actually needs the
// full context of string, line_index, and line.
#[derive(Clone, Debug)]
pub struct Utf8StringTermLineElemCharView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    pub string: &'a str,
    pub line_index: usize,

    // cached values
    pub line_count: usize,
    pub line: &'a str,
    pub line_char_count: usize,
}

impl<'a> Utf8StringTermLineElemCharView<'a> {
    pub fn new(string: &'a str, line_index: usize) -> Result<Self> {
        let mut line_i = st::split_inclusive_allow_trailing_empty(string, '\n');
        let line_count = line_i.clone().count();
        let line = line_i.nth(line_index).ok_or_else(|| {
            anyhow::anyhow!("Utf8StringTermLineElemCharView line_index out of bounds")
        })?;
        let line_char_count = line.chars().count();
        Ok(Self {
            string,
            line_index,
            line_count,
            line,
            line_char_count,
        })
    }
    /// In some cases, the line is already known, so this can be used to avoid calling split_inclusive.
    pub fn new_with_cached_line(
        string: &'a str,
        line_index: usize,
        line_count: usize,
        line: &'a str,
        line_char_count: usize,
    ) -> Result<Self> {
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').count(),
            line_count,
            "programmer error: actual line_count does not match specified line_count",
        );
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').nth(line_index),
            Some(line),
            "programmer error: actual line does not match specified cached line"
        );
        assert_eq!(
            line.chars().count(),
            line_char_count,
            "programmer error: actual line_char_count does not match specified line_char_count"
        );
        Ok(Self {
            string,
            line_index,
            line_count,
            line,
            line_char_count,
        })
    }
}

impl<'b> qv::QueryTrait for Utf8StringTermLineElemCharView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            let char_o = self.line.chars().nth(*element_index as usize);
            Box::new(
                qv::Utf8StringTermLineElemCharElemView::new_with_cached_line_and_char(
                    self.string,
                    self.line_index,
                    *element_index as usize,
                    self.line_count,
                    self.line,
                    self.line_char_count,
                    char_o,
                )?,
            )
            .run_query(&mut address_token_i)
        } else {
            // TODO: Support "len" query.
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineElemCharView query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalTrait for Utf8StringTermLineElemCharView<'b> {
    /// This will produce an ArrayTerm populated with the chars of the line.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        use dy::IntoValue;
        let char_v = self
            .line
            .chars()
            .map(|c| c.into_value())
            .collect::<Vec<dy::Value>>();
        let chars = dy::ArrayTerm::from(char_v);
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(chars.into_value()),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum Utf8StringTermLineElemCharViewQuery<'a> {
    Utf8StringTermLineElemCharElemView(qv::Utf8StringTermLineElemCharElemView<'a>),
}

impl<'b> qv::SingleQuery<dy::Value> for Utf8StringTermLineElemCharView<'b> {
    type ReturnType<'a> = Utf8StringTermLineElemCharViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(char_index) = address_token.downcast_ref::<u32>() {
            let char_index = *char_index as usize;
            Ok(
                qv::Utf8StringTermLineElemCharElemView::new_with_cached_line_and_char(
                    self.string,
                    self.line_index,
                    char_index,
                    self.line_count,
                    self.line,
                    self.line_char_count,
                    self.line.chars().nth(char_index),
                )?
                .into(),
            )
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineElemCharView::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}
