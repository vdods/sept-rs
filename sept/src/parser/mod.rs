mod expr;
mod expr_sequence;
mod parse;
mod terminal;
mod tuple_expr;

pub use crate::parser::{
    expr::{Expr, ExprKind},
    expr_sequence::ExprSequence,
    parse::{ExprSequenceEnd, ParseStats, parse_deconstruction, parse_value},
    terminal::Terminal,
    tuple_expr::TupleExpr,
};

#[macro_export]
macro_rules! make_expr_sequence {
    // Base case
    () => {
        $crate::parser::ExprSequence::from(vec![])
    };
    // At least one arg, optional trailing comma.
    ($first_expr:expr $(, $rest_expr:expr)* $(,)?) => {
        $crate::parser::ExprSequence::from(vec![
            $crate::parser::Expr::from($first_expr),
            $(
                $crate::parser::Expr::from($rest_expr),
            )*
        ])
    };
}

#[macro_export]
macro_rules! make_tuple_expr {
    // Base case
    () => {
        $crate::parser::TupleExpr::from(vec![])
    };
    // At least one arg, optional trailing comma.
    ($first_expr:expr $(, $rest_expr:expr)* $(,)?) => {
        $crate::parser::TupleExpr::from(vec![
            $crate::parser::Expr::from($first_expr),
            $(
                $crate::parser::Expr::from($rest_expr),
            )*
        ])
    };
}

/// First argument must be the name of a variant of scanner::Token, e.g. CIdentifier.
#[macro_export]
macro_rules! make_terminal {
    // optional trailing comma.
    ($kind:ident, $s:expr $(,)?) => {
        $crate::parser::Terminal::from($crate::scanner::Token::from($crate::scanner::$kind::from($s)))
    };
}
