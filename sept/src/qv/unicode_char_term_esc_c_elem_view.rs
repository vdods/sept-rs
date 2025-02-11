use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

/// This is a view for a specific element of the single-char escape representation of a char that corresponds to
/// a single-char escape.  The possible values are '\0', '\n', '\t', '\r', '\"', '\'', '\\'.
#[derive(Clone, Debug)]
pub struct UnicodeCharTermEscCElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a char view object, or Box<dyn Borrow<char>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    #[allow(dead_code)]
    pub c: &'a st::UnicodeCharTerm,
    pub char_index: usize,

    // Cached values
    indexed_char_v: &'static [char; 2],
}

impl<'a> UnicodeCharTermEscCElemView<'a> {
    pub fn new(c: &'a st::UnicodeCharTerm, char_index: usize) -> Result<Self> {
        let indexed_char_v = qv::UnicodeCharTermEscCView::make_indexed_char_v(*c)?;
        Ok(Self {
            c,
            char_index,
            indexed_char_v,
        })
    }
    pub fn new_with_cached_indexed_chars(
        c: &'a st::UnicodeCharTerm,
        char_index: usize,
        indexed_char_v: &'static [char; 2],
    ) -> Result<Self> {
        Ok(Self {
            c,
            char_index,
            indexed_char_v,
        })
    }
    pub fn indexed_char_count(&self) -> usize {
        self.indexed_char_v.len()
    }
    pub fn go_home(&mut self) {
        self.char_index = 0;
    }
    pub fn go_end(&mut self) {
        self.char_index = self.indexed_char_v.len();
        assert!(self.indexed_char_v.get(self.char_index) == None);
    }
    pub fn increment_char_index_by(&mut self, char_index_delta: isize) {
        self.char_index = self
            .char_index
            .saturating_add_signed(char_index_delta)
            .min(self.indexed_char_v.len());
    }
}

impl<'b> qv::QueryT for UnicodeCharTermEscCElemView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        // TODO: If char ever gets further queries (e.g. numeric unicode value), then pass them on here.
        use st::StringifiableT;
        anyhow::bail!(
            "UnicodeCharTermEscCElemView query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalT for UnicodeCharTermEscCElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        use dy::IntoValueT;
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(if let Some(c) = self.indexed_char_v.get(self.char_index) {
                c.into_value()
            } else {
                // NOTE: Returning Void is a bit of a hack.  Maybe there should be some "End" NPTerm instead.
                st::Void.into_value()
            }),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum UnicodeCharTermCharElemViewQuery {}

impl<'b> qv::SingleQueryT<dy::Value> for UnicodeCharTermEscCElemView<'b> {
    type ReturnType<'a> = UnicodeCharTermCharElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        // TODO: If this view ever gets any queries, forward them here.
        anyhow::bail!("UnicodeCharTermEscCElemView::run_single_query does not support any queries");
    }
}
