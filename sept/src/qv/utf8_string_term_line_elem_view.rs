use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Utf8StringTermLineElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    // TODO: Eventually allow one-past-the-end.
    pub string: &'a str,
    pub line_index: usize,

    // cached values
    pub line_count: usize,
    pub line: &'a str,
    pub line_char_count: usize,
}

impl<'a> Utf8StringTermLineElemView<'a> {
    pub fn new(string: &'a str, line_index: usize) -> Result<Self> {
        let mut line_i = st::split_inclusive_allow_trailing_empty(string, '\n');
        let line_count = line_i.clone().count();
        let line = line_i.nth(line_index).ok_or_else(|| {
            anyhow::anyhow!("Utf8StringTermLineElemView line_index out of bounds")
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
            "programmer error: actual line_count does not match specified line_count"
        );
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').nth(line_index),
            Some(line),
            "programmer error: actual line does not match specified cached line"
        );
        assert_eq!(
            line.chars().count(),
            line_char_count,
            "programmer error: actual line_char_count does not match specified cached line_char_count"
        );
        Ok(Self {
            string,
            line_index,
            line_count,
            line,
            line_char_count,
        })
    }
    pub fn go_home(&mut self) {
        self.set_line_index(0);
    }
    pub fn go_end(&mut self) {
        assert!(self.line_count > 0);
        self.set_line_index(self.line_count - 1);
    }
    pub fn increment_line_index_by(&mut self, line_index_delta: isize) {
        assert!(self.line_count > 0);
        self.set_line_index(
            self.line_index
                .saturating_add_signed(line_index_delta)
                .min(self.line_count - 1),
        );
    }
    /// This also updates the cached values.
    fn set_line_index(&mut self, line_index: usize) {
        if line_index >= self.line_count {
            panic!("programmer error: line_index out of bounds");
        }
        self.line_index = line_index;
        let mut line_i = st::split_inclusive_allow_trailing_empty(self.string, '\n');
        self.line_count = line_i.clone().count();
        self.line = line_i.nth(self.line_index).unwrap();
        self.line_char_count = self.line.chars().count();
    }
}

impl<'b> qv::QueryTrait for Utf8StringTermLineElemView<'b> {
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
        match first_address.downcast_ref::<String>().map(String::as_str) {
            Some("char") => {
                // anyhow::ensure!(address_token_i.peek().is_some(), "Utf8StringTermLineView query address 'char' requires a second address (char index) but none was provided");
                // let second_address = address_token_i.next().unwrap();
                // anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTermLineView query address 'char' requires a second address (char index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                // let char_index = *second_address.downcast_ref::<u32>().unwrap();
                // let utf8_string_line_char_view = qv::Utf8StringTermLineElemCharElemView::new(
                //     self.string,
                //     self.line_index,
                //     char_index as usize,
                // )?;
                // // Pass on the rest of the address to the char view's impl of query_mut.
                // Box::new(utf8_string_line_char_view).run_query(&mut address_token_i)
                Box::new(qv::Utf8StringTermLineElemCharView::new_with_cached_line(
                    self.string,
                    self.line_index,
                    self.line_count,
                    self.line,
                    self.line_char_count,
                )?)
                .run_query(&mut address_token_i)
            }
            _ => {
                use st::Stringifiable;
                anyhow::bail!(
                    "Utf8StringTerm query doesn't support address: {}",
                    first_address.stringify()
                )
            }
        }
    }
}

impl<'b> qv::EvalTrait for Utf8StringTermLineElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(dy::Value::from(self.line.to_string()).into()),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum Utf8StringTermLineElemViewQuery<'a> {
    Utf8StringTermLineElemCharView(qv::Utf8StringTermLineElemCharView<'a>),
}

impl<'b> qv::SingleQuery<dy::Value> for Utf8StringTermLineElemView<'b> {
    type ReturnType<'a> = Utf8StringTermLineElemViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(address_string) = address_token.downcast_ref::<String>() {
            match address_string.as_str() {
                "char" => Ok(qv::Utf8StringTermLineElemCharView::new(
                    self.string,
                    self.line_index,
                )?
                .into()),
                // TODO: "len" perhaps
                _ => {
                    anyhow::bail!(
                        "Utf8StringTermLineElemView::run_single_query; unrecognized address_token {:?}",
                        address_string.as_str()
                    );
                }
            }
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineElemView::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}
