use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

/// This is a view for a char that corresponds to a single-char escape.  The possible values are
/// '\0', '\n', '\t', '\r', '\"', '\'', '\\'.
#[derive(Clone, Debug)]
pub struct UnicodeCharTermEscCView<'a> {
    pub c: &'a st::UnicodeCharTerm,
    // Cached values
    // TODO: This should actually be an ASCII string of printable chars.
    pub indexed_char_v: &'static [char; 2],
}

impl<'a> UnicodeCharTermEscCView<'a> {
    pub fn new(c: &'a st::UnicodeCharTerm) -> Result<Self> {
        // Verify that the char is a single-char escape.
        let indexed_char_v = Self::make_indexed_char_v(*c)?;
        Ok(Self { c, indexed_char_v })
    }
    pub fn indexed_char_count() -> usize {
        2
    }
    pub fn make_indexed_char_v(c: char) -> Result<&'static [char; 2]> {
        const BACKSLASH_0: [char; 2] = ['\\', '0'];
        const BACKSLASH_T: [char; 2] = ['\\', 't'];
        const BACKSLASH_N: [char; 2] = ['\\', 'n'];
        const BACKSLASH_R: [char; 2] = ['\\', 'r'];
        const BACKSLASH_DOUBLE_QUOTE: [char; 2] = ['\\', '"'];
        const BACKSLASH_SINGLE_QUOTE: [char; 2] = ['\\', '\''];
        const BACKSLASH_BACKSLASH: [char; 2] = ['\\', '\\'];

        match c {
            '\0' => Ok(&BACKSLASH_0),
            '\t' => Ok(&BACKSLASH_T),
            '\n' => Ok(&BACKSLASH_N),
            '\r' => Ok(&BACKSLASH_R),
            '\"' => Ok(&BACKSLASH_DOUBLE_QUOTE),
            '\'' => Ok(&BACKSLASH_SINGLE_QUOTE),
            '\\' => Ok(&BACKSLASH_BACKSLASH),
            _ => {
                anyhow::bail!(
                    "char {} does not correspond to a single-char escape",
                    c.escape_debug()
                );
            }
        }
    }
}

impl<'b> qv::QueryT for UnicodeCharTermEscCView<'b> {
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
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            Box::new(
                qv::UnicodeCharTermEscCElemView::new_with_cached_indexed_chars(
                    self.c,
                    *element_index as usize,
                    self.indexed_char_v,
                )?,
            )
            .run_query(&mut address_token_i)
        } else {
            use st::StringifiableT;
            anyhow::bail!(
                "UnicodeCharTermEscCView query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalT for UnicodeCharTermEscCView<'b> {
    /// This will produce an ArrayTerm populated with the chars of the string.
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // TODO: Make this not allocate, return a ref to a const value.
        let chars = dy::ArrayTerm::from(
            self.indexed_char_v
                .iter()
                .map(|c| (*c).into_value())
                .collect::<Vec<dy::Value>>(),
        );
        use dy::IntoValueT;
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(chars.into_value()),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum UnicodeCharTermEscCViewQuery<'a> {
    UnicodeCharTermEscCElemView(qv::UnicodeCharTermEscCElemView<'a>),
}

impl<'a> From<UnicodeCharTermEscCViewQuery<'a>> for Box<dyn qv::EvalT + 'a> {
    fn from(value: UnicodeCharTermEscCViewQuery<'a>) -> Self {
        match value {
            UnicodeCharTermEscCViewQuery::UnicodeCharTermEscCElemView(x) => Box::new(x),
        }
    }
}

impl<'b> qv::SingleQueryT<dy::Value> for UnicodeCharTermEscCView<'b> {
    type ReturnType<'a> = UnicodeCharTermEscCViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(char_index) = address_token.downcast_ref::<u32>() {
            Ok(
                qv::UnicodeCharTermEscCElemView::new_with_cached_indexed_chars(
                    self.c,
                    *char_index as usize,
                    self.indexed_char_v,
                )?
                .into(),
            )
        } else {
            use st::StringifiableT;
            anyhow::bail!(
                "UnicodeCharTermEscCView::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}
