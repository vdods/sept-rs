use crate::parser::{ExprSequence, Terminal, TupleExpr};

#[derive(Debug, derive_more::From, derive_more::TryInto)]
pub enum Expr<'a> {
    Terminal(Terminal<'a>),
    ExprSequence(ExprSequence<'a>),
    TupleExpr(TupleExpr<'a>),
}

impl<'a> Expr<'a> {
    // TODO: Use enum_kind instead to derive this.
    pub fn kind(&self) -> ExprKind {
        match self {
            Expr::Terminal(_) => ExprKind::Terminal,
            Expr::ExprSequence(_) => ExprKind::ExprSequence,
            Expr::TupleExpr(_) => ExprKind::TupleExpr,
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(self, Expr::Terminal(_))
    }
    pub fn is_expr_sequence(&self) -> bool {
        matches!(self, Expr::ExprSequence(_))
    }
    pub fn is_tuple_expr(&self) -> bool {
        matches!(self, Expr::TupleExpr(_))
    }
    pub fn as_terminal(&self) -> Option<&Terminal<'a>> {
        bind_match::bind_match!(self, Expr::Terminal(terminal) => terminal)
    }
    pub fn as_expr_sequence(&self) -> Option<&ExprSequence<'a>> {
        bind_match::bind_match!(self, Expr::ExprSequence(expr_sequence) => expr_sequence)
    }
    pub fn as_tuple_expr(&self) -> Option<&TupleExpr<'a>> {
        bind_match::bind_match!(self, Expr::TupleExpr(tuple_expr) => tuple_expr)
    }
}

// TODO: Derive this
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExprKind {
    Terminal,
    ExprSequence,
    TupleExpr,
}
