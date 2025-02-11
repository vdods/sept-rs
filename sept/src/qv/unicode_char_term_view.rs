use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct UnicodeCharTermView<'a> {
    pub c: &'a st::UnicodeCharTerm,
}

impl<'a> UnicodeCharTermView<'a> {
    pub fn new(c: &'a st::UnicodeCharTerm) -> Self {
        Self { c }
    }
    pub fn is_plain_char(c: char) -> bool {
        // Unicode code point value of c.
        let n = c as u32;
        // The "plain" chars are defined to be the printable ASCII chars, except for the single-char escapes.
        // See UnicodeCharTermEscCView for details on the single-char escapes.
        match n {
            0x20..=0x7E => !Self::is_single_char_escape(c),
            _ => false,
        }
    }
    pub fn is_single_char_escape(c: char) -> bool {
        match c {
            '\0' | '\t' | '\n' | '\r' | '\"' | '\'' | '\\' => true,
            _ => false,
        }
    }
    pub fn unescape_single_char(c: char) -> Option<char> {
        match c {
            '0' => Some('\0'),
            't' => Some('\t'),
            'n' => Some('\n'),
            'r' => Some('\r'),
            '\"' => Some('\"'),
            '\'' => Some('\''),
            '\\' => Some('\\'),
            _ => None,
        }
    }
}

impl<'b> qv::QueryT for UnicodeCharTermView<'b> {
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
            "UnicodeCharTermView query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalT for UnicodeCharTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.c))
    }
}
