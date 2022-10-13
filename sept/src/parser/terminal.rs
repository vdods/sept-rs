use crate::scanner;

/// A terminal token, e.g. scanner::Token::CIdentifier, DecimalPointLiteral, etc.
#[derive(derive_more::AsRef, Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct Terminal<'a>(scanner::Token<'a>);
