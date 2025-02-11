use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

/// A View for a "plain" char, which are defined to be chars that are not escaped.  Plain chars are represented directly.
#[derive(Clone, Debug)]
pub struct UnicodeCharTermPlainView<'a> {
    pub c: &'a st::UnicodeCharTerm,
}

impl<'a> UnicodeCharTermPlainView<'a> {
    pub fn new(c: &'a st::UnicodeCharTerm) -> Result<Self> {
        anyhow::ensure!(
            qv::UnicodeCharTermView::is_plain_char(*c),
            "char {} is not a plain char",
            c.escape_debug()
        );
        Ok(Self { c })
    }
    pub fn indexed_char_count() -> usize {
        1
    }
}

impl<'b> qv::QueryT for UnicodeCharTermPlainView<'b> {
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
        // TODO: Support "forced-escape" query.
        use st::StringifiableT;
        anyhow::bail!(
            "UnicodeCharTermPlainView query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalT for UnicodeCharTermPlainView<'b> {
    /// This will produce an ArrayTerm populated with the chars of the string.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        use dy::IntoValueT;
        let char_v = vec![(*self.c).into_value()];
        let chars = dy::ArrayTerm::from(char_v);
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(chars.into_value()),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum UnicodeCharTermPlainViewQuery<'a> {
    UnicodeCharTermPlainViewElemView(qv::UnicodeCharTermPlainElemView<'a>),
}

impl<'a> From<UnicodeCharTermPlainViewQuery<'a>> for Box<dyn qv::EvalT + 'a> {
    fn from(value: UnicodeCharTermPlainViewQuery<'a>) -> Self {
        match value {
            UnicodeCharTermPlainViewQuery::UnicodeCharTermPlainViewElemView(x) => Box::new(x),
        }
    }
}

impl<'b> qv::SingleQueryT<dy::Value> for UnicodeCharTermPlainView<'b> {
    type ReturnType<'a> = UnicodeCharTermPlainViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(char_index) = address_token.downcast_ref::<u32>() {
            Ok(
                UnicodeCharTermPlainViewQuery::UnicodeCharTermPlainViewElemView(
                    qv::UnicodeCharTermPlainElemView::new(self.c, *char_index as usize)?,
                ),
            )
        } else {
            use st::StringifiableT;
            anyhow::bail!(
                "UnicodeCharTermPlainView::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}
