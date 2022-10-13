use crate::parser::{ExprSequence, Terminal, TupleExpr};

#[derive(Debug, derive_more::From, enum_as_inner::EnumAsInner, enum_kinds::EnumKind, PartialEq)]
#[enum_kind(ExprKind)]
pub enum Expr<'a> {
    Terminal(Terminal<'a>),
    ExprSequence(ExprSequence<'a>),
    TupleExpr(TupleExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn kind(&self) -> ExprKind {
        self.into()
    }
}
