use crate::parser::Expr;

/// A sequence of Expr, not separated by commas.
#[derive(derive_more::AsRef, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct ExprSequence<'a>(Vec<Expr<'a>>);
