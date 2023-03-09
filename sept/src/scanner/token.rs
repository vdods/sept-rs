use crate::Result;

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct Whitespace<'a>(&'a str);

impl<'a> Whitespace<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct CIdentifier<'a>(&'a str);

impl<'a> CIdentifier<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct DecimalPointLiteral<'a>(&'a str);

impl<'a> DecimalPointLiteral<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct IntegerLiteral<'a>(&'a str);

impl<'a> IntegerLiteral<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct AsciiStringLiteral<'a>(&'a str);

impl<'a> AsciiStringLiteral<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
    /// This will turn an escaped string "\"...\"" into an unescaped string, where escape codes are
    /// transformed into the chars they represent.
    // TODO: Implement this better by implementing an iterator for "next char slice" which will account
    // for escaped chars.
    pub fn unescaped(&self) -> Result<String> {
        assert!(
            self.len() >= 2 && self.starts_with('"') && self.ends_with('"'),
            "expected the AsciiStringLiteral to have the form \"...\""
        );
        // TODO: Pre-compute the length of the result to pre-allocate it.
        let mut retval = String::new();
        // This is a slice to the interior bytes of the string, in particular, not including the
        // enclosing outer double-quotes.
        let str_as_u8_slice: &[u8] = self.0.as_ref();
        let str_interior_as_u8_slice: &[u8] = &str_as_u8_slice[1..self.len() - 1];
        let mut cursor = 0usize;
        while cursor < str_interior_as_u8_slice.len() {
            let first_char = str_interior_as_u8_slice[cursor] as char;
            let c = match first_char {
                '\\' => {
                    cursor += 1;
                    anyhow::ensure!(
                        cursor < str_interior_as_u8_slice.len(),
                        "malformed AsciiStringLiteral; expected escape char after \\"
                    );
                    let second_char = str_interior_as_u8_slice[cursor] as char;
                    match second_char {
                        '0' => {
                            cursor += 1;
                            '\0'
                        }
                        'a' => {
                            cursor += 1;
                            '\x07'
                        } // \a
                        'b' => {
                            cursor += 1;
                            '\x08'
                        } // \b
                        't' => {
                            cursor += 1;
                            '\t'
                        }
                        'n' => {
                            cursor += 1;
                            '\n'
                        }
                        'v' => {
                            cursor += 1;
                            '\x0B'
                        } // \v
                        'f' => {
                            cursor += 1;
                            '\x0C'
                        } // \f
                        'r' => {
                            cursor += 1;
                            '\r'
                        }
                        '"' => {
                            cursor += 1;
                            '"'
                        }
                        '\\' => {
                            cursor += 1;
                            '\\'
                        }
                        'x' => {
                            cursor += 1;
                            anyhow::ensure!(
                                cursor + 2 <= str_interior_as_u8_slice.len(),
                                "malformed AsciiStringLiteral; expected two hex digits after \\x"
                            );
                            let first_hex_char = str_interior_as_u8_slice[cursor] as char;
                            let second_hex_char = str_interior_as_u8_slice[cursor] as char;
                            anyhow::ensure!(first_hex_char.is_ascii_hexdigit() && second_hex_char.is_ascii_hexdigit(), "malformed AsciiStringLiteral; expected two hex digits after \\x but got {:?}", &str_interior_as_u8_slice[cursor..cursor+2]);
                            let unescaped_hex_char = str_interior_as_u8_slice[cursor] * 0x10u8
                                + str_interior_as_u8_slice[cursor + 1];
                            cursor += 2;
                            unescaped_hex_char as char
                        }
                        _ => {
                            anyhow::bail!(
                                "malformed AsciiStringLiteral; invalid escape char code: {:?}",
                                second_char
                            );
                        }
                    }
                }
                '\0' | '\x07' | '\x08' | '\t' | '\n' | '\x0B' | '\x0C' | '\r' | '"' => {
                    anyhow::bail!(
                        "malformed AsciiStringLiteral; found an unescaped char: {:?}",
                        first_char
                    );
                }
                c => {
                    cursor += 1;
                    c
                }
            };
            retval.push(c);
        }
        Ok(retval)
    }
}

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct UnrecognizedInput<'a>(&'a str);

// TODO: Could include line number, and maybe char range within line
/// Note that these have to be in order that the matches will be tried in.
/// Start with the quick/simple ones, then go to the slow/complex ones.  It's important
/// that try_scanning_decimal_point_literal happens before try_scanning_integer_literal.
#[derive(Clone, Debug, enum_kinds::EnumKind, derive_more::From, PartialEq)]
#[enum_kind(TokenKind, derive(enum_map::Enum))]
pub enum Token<'a> {
    EndOfInput,
    OpenParen,
    CloseParen,
    Comma,
    Whitespace(Whitespace<'a>),
    CIdentifier(CIdentifier<'a>),
    DecimalPointLiteral(DecimalPointLiteral<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
    AsciiStringLiteral(AsciiStringLiteral<'a>),
    UnrecognizedInput(UnrecognizedInput<'a>),
}

impl<'a> Token<'a> {
    pub fn new(token_kind: TokenKind, input: &'a str) -> Self {
        match token_kind {
            TokenKind::EndOfInput => Token::EndOfInput,
            TokenKind::OpenParen => Token::OpenParen,
            TokenKind::CloseParen => Token::CloseParen,
            TokenKind::Comma => Token::Comma,
            TokenKind::Whitespace => Token::Whitespace(Whitespace(input)),
            TokenKind::CIdentifier => Token::CIdentifier(CIdentifier(input)),
            TokenKind::DecimalPointLiteral => {
                Token::DecimalPointLiteral(DecimalPointLiteral(input))
            }
            TokenKind::IntegerLiteral => Token::IntegerLiteral(IntegerLiteral(input)),
            TokenKind::AsciiStringLiteral => Token::AsciiStringLiteral(AsciiStringLiteral(input)),
            TokenKind::UnrecognizedInput => Token::UnrecognizedInput(UnrecognizedInput(input)),
        }
    }
    pub fn kind(&self) -> TokenKind {
        self.into()
    }
}
