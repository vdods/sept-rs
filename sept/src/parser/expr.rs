use crate::scanner;

//
// Dynamic syntactical expression type
//

#[derive(Debug, derive_more::From, enum_as_inner::EnumAsInner, enum_kinds::EnumKind, PartialEq)]
#[enum_kind(ExprKind)]
pub enum Expr<'a> {
    ExprSequence(ExprSequence<'a>),
    Syntactuple(Syntactuple<'a>),
    Terminal(Terminal<'a>),
}

impl<'a> Expr<'a> {
    pub fn kind(&self) -> ExprKind {
        self.into()
    }
}

//
// Specific syntactical types
//

/// A sequence of Expr, not separated by commas.
#[derive(
    derive_more::AsRef, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq,
)]
pub struct ExprSequence<'a>(Vec<Expr<'a>>);

/// A sequence of Expr, separated by commas, and enclosed by parens.
// TODO: Consider making the inner type ExprSequence
#[derive(
    derive_more::AsRef, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq,
)]
pub struct Syntactuple<'a>(Vec<Expr<'a>>);

/// A terminal token, e.g. scanner::Token::CIdentifier, DecimalPointLiteral, etc.
#[derive(
    derive_more::AsRef,
    Clone,
    Debug,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    PartialEq,
)]
pub struct Terminal<'a>(scanner::Token<'a>);

//
// Macro defs
//

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
macro_rules! make_syntactuple {
    // Base case
    () => {
        $crate::parser::Syntactuple::from(vec![])
    };
    // At least one arg, optional trailing comma.
    ($first_expr:expr $(, $rest_expr:expr)* $(,)?) => {
        $crate::parser::Syntactuple::from(vec![
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
        $crate::parser::Terminal::from($crate::scanner::Token::from($crate::scanner::$kind::from(
            $s,
        )))
    };
}
