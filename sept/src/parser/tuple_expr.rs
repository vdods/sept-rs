use crate::parser::Expr;

/// A sequence of Expr, separated by commas, and enclosed by parens.
// TODO: Consider renaming this to Syntactuple or SyntaxTuple, since it only represents syntax
// and is distinct from dy::TupleTerm.
// TODO: Consider making the inner type ExprSequence
#[derive(derive_more::AsRef, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct TupleExpr<'a>(Vec<Expr<'a>>);
