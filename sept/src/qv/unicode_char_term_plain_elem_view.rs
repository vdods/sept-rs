use crate::{dy, qv, st, Error, Result};

/// A View for an element of a "plain" char, which are defined to be chars that are not escaped.  Plain chars
/// are represented directly, so there's only one element.
#[derive(Clone, Debug)]
pub struct UnicodeCharTermPlainElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a char view object, or Box<dyn Borrow<char>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    #[allow(dead_code)]
    pub c: &'a st::UnicodeCharTerm,
    /// char_index is really only used for determining if the cursor is "on" the char, or "after" it for editing purposes.
    pub char_index: usize,
}

impl<'a> UnicodeCharTermPlainElemView<'a> {
    pub fn new(c: &'a st::UnicodeCharTerm, char_index: usize) -> Result<Self> {
        anyhow::ensure!(
            char_index <= Self::indexed_char_count(),
            "UnicodeCharTermPlainElemView char index out of bounds"
        );
        Ok(Self { c, char_index })
    }
    pub fn indexed_char_count() -> usize {
        1
    }
    pub fn go_home(&mut self) {
        self.char_index = 0;
    }
    pub fn go_end(&mut self) {
        self.char_index = Self::indexed_char_count();
    }
    pub fn increment_char_index_by(&mut self, char_index_delta: isize) {
        self.char_index = self
            .char_index
            .saturating_add_signed(char_index_delta)
            .min(Self::indexed_char_count());
    }
}

impl<'b> qv::QueryT for UnicodeCharTermPlainElemView<'b> {
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
            "UnicodeCharTermPlainElemView query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalT for UnicodeCharTermPlainElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.c))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum UnicodeCharTermPlainElemViewQuery {}

impl<'b> qv::SingleQueryT<dy::Value> for UnicodeCharTermPlainElemView<'b> {
    type ReturnType<'a> = UnicodeCharTermPlainElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        // TODO: If this view ever gets any queries, forward them here.
        anyhow::bail!(
            "UnicodeCharTermPlainElemView::run_single_query does not support any queries"
        );
    }
}
