use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct UTF8StringTermCharElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    pub string: &'a str,
    pub char_index: usize,

    // Cached values
    pub char_count: usize,
    pub char_o: Option<char>,
}

impl<'a> UTF8StringTermCharElemView<'a> {
    pub fn new(string: &'a str, char_index: usize) -> Result<Self> {
        let char_count = string.chars().count();
        anyhow::ensure!(
            char_index <= char_count,
            "UTF8StringTermCharElementView char index out of bounds"
        );
        let char_o = string.chars().nth(char_index);
        Ok(Self {
            string,
            char_index,
            char_count,
            char_o,
        })
    }
    pub fn go_home(&mut self) {
        self.char_index = 0;
        self.char_o = self.string.chars().nth(self.char_index);
    }
    pub fn go_end(&mut self) {
        self.char_index = self.char_count;
        assert!(self.string.chars().nth(self.char_index) == None);
        self.char_o = None;
    }
    pub fn increment_char_index_by(&mut self, char_index_delta: isize) {
        self.char_index = self
            .char_index
            .saturating_add_signed(char_index_delta)
            .min(self.char_count);
        self.char_o = self.string.chars().nth(self.char_index);
    }
}

impl<'b> qv::QueryT for UTF8StringTermCharElemView<'b> {
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
            "UTF8StringTermCharView query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalT for UTF8StringTermCharElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        use dy::IntoValueT;
        // We already retrieved the char during construction, so just return the value.
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(if let Some(c) = self.char_o {
                c.into_value()
            } else {
                // NOTE: Returning Void is a bit of a hack.  Maybe there should be some "End" NPTerm instead.
                st::Void.into_value()
            }),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum UTF8StringTermCharElemViewQuery {}

impl<'b> qv::SingleQueryT<dy::Value> for UTF8StringTermCharElemView<'b> {
    type ReturnType<'a> = UTF8StringTermCharElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        // TODO: Once char has queries, forward to that.
        anyhow::bail!("UTF8StringTermCharElemView::run_single_query does not support any queries");
    }
}
