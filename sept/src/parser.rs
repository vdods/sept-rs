use crate::{Result, scanner};

/// A terminal token, e.g. scanner::Token::CIdentifier, DecimalLiteral, etc.
#[derive(derive_more::AsRef, Clone, Debug, derive_more::From, derive_more::Into)]
pub struct Terminal<'a>(scanner::Token<'a>);

/// A sequence of Expr, not separated by commas.
#[derive(derive_more::AsRef, Debug, derive_more::From, derive_more::Into)]
pub struct ExprSequence<'a>(Vec<Expr<'a>>);

/// A sequence of Expr, separated by commas, and enclosed by parens.
#[derive(derive_more::AsRef, Debug, derive_more::From, derive_more::Into)]
pub struct TupleExpr<'a>(Vec<Expr<'a>>);

#[derive(Debug)]
pub enum Expr<'a> {
    Terminal(Terminal<'a>),
    ExprSequence(ExprSequence<'a>),
    TupleExpr(TupleExpr<'a>),
}

// TODO: Derive this
impl<'a> From<Terminal<'a>> for Expr<'a> {
    fn from(terminal: Terminal<'a>) -> Self {
        Expr::Terminal(terminal)
    }
}

// TODO: Derive this
impl<'a> From<ExprSequence<'a>> for Expr<'a> {
    fn from(expr_sequence: ExprSequence<'a>) -> Self {
        Expr::ExprSequence(expr_sequence)
    }
}

// TODO: Derive this
impl<'a> From<TupleExpr<'a>> for Expr<'a> {
    fn from(tuple_expr: TupleExpr<'a>) -> Self {
        Expr::TupleExpr(tuple_expr)
    }
}

#[derive(Debug)]
pub struct ParseStats {
    // TODO: Maybe track the line or range
    pub scanner_token_count: usize,
}

pub fn parse_terminal<'a>(token_v: &[scanner::Token<'a>]) -> Result<(Terminal<'a>, ParseStats)> {
    log::trace!("parse_terminal; token_v ({} elements): {:?}", token_v.len(), token_v);
    anyhow::ensure!(!token_v.is_empty(), "Can't parse empty token_v");

    let next_token = &token_v[0];
    log::trace!("    parse_terminal; next_token {:?}", next_token);
    match next_token.kind() {
        scanner::TokenKind::CIdentifier |
        scanner::TokenKind::DecimalLiteral |
        scanner::TokenKind::IntegerLiteral |
        scanner::TokenKind::AsciiStringLiteral => {
            Ok((Terminal::from(next_token.clone()), ParseStats { scanner_token_count: 1 }))
        }
        _ => {
            // TODO: More context, line number, etc.
            Err(anyhow::anyhow!("Unexpected input: {:?}", next_token))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExprSequenceEnd {
    OnlyOnEndOfInput,
    Unrestricted,
}

pub fn parse_expr_sequence<'a>(
    token_v: &[scanner::Token<'a>],
    expr_sequence_end: ExprSequenceEnd,
) -> Result<(ExprSequence<'a>, ParseStats)> {
    log::trace!("parse_expr_sequence; token_v ({} elements): {:?}", token_v.len(), token_v);
    anyhow::ensure!(!token_v.is_empty(), "Can't parse empty token_v");

    let mut scanner_token_count = 0usize;
    let mut expr_v: Vec<Expr<'a>> = Vec::new();
    loop {
        assert!(scanner_token_count <= token_v.len());
        // Interpret scanner_token_count == token_v.len() as EndOfInput.
        if scanner_token_count == token_v.len() {
            // This always gracefully ends ExprSequence.
            return Ok((ExprSequence::from(expr_v), ParseStats { scanner_token_count }));
        }

        let next_token = &token_v[scanner_token_count];
        log::trace!("    parse_expr_sequence; scanner_token_count: {}, next_token: {:?}", scanner_token_count, next_token);
        match next_token.kind() {
            scanner::TokenKind::EndOfInput => {
                // This always gracefully ends ExprSequence.
                return Ok((ExprSequence::from(expr_v), ParseStats { scanner_token_count }));
            }

            scanner::TokenKind::CloseParen |
            scanner::TokenKind::Comma => {
                // These conditionally gracefully end ExprSequence; they're for TupleExpr to parse.
                if expr_sequence_end == ExprSequenceEnd::Unrestricted {
                    return Ok((ExprSequence::from(expr_v), ParseStats { scanner_token_count }));
                } else {
                    // TODO: line number, more context, etc.
                    anyhow::bail!("expected EndOfInput, but found other input");
                }
            }

            scanner::TokenKind::OpenParen => {
                let (tuple_expr, parse_stats) = parse_tuple_expr(&token_v[scanner_token_count..])?;
                assert_eq!(token_v[scanner_token_count + parse_stats.scanner_token_count - 1].kind(), scanner::TokenKind::CloseParen);
                expr_v.push(tuple_expr.into());
                scanner_token_count += parse_stats.scanner_token_count;
            }

            scanner::TokenKind::CIdentifier |
            scanner::TokenKind::DecimalLiteral |
            scanner::TokenKind::IntegerLiteral |
            scanner::TokenKind::AsciiStringLiteral => {
                let (terminal, parse_stats) = parse_terminal(&token_v[scanner_token_count..])?;
                expr_v.push(terminal.into());
                scanner_token_count += parse_stats.scanner_token_count;
            }

            scanner::TokenKind::Whitespace => {
                panic!("This shouldn't be possible.");
            }

            scanner::TokenKind::UnrecognizedInput => {
                // TODO: Context, line number, etc.
                anyhow::bail!("unrecognized input");
            }
        }
    }
}

pub fn parse_tuple_expr<'a>(token_v: &[scanner::Token<'a>]) -> Result<(TupleExpr<'a>, ParseStats)> {
    log::trace!("parse_tuple_expr; token_v ({} elements): {:?}", token_v.len(), token_v);
    anyhow::ensure!(!token_v.len() >= 2, "token_v is too short (expected at least 2 elements, got {})", token_v.len());

    assert_eq!(token_v[0].kind(), scanner::TokenKind::OpenParen);
    // Start after the OpenParen.
    let mut scanner_token_count = 1usize;
    let mut expr_v: Vec<Expr<'a>> = Vec::new();
    loop {
        let next_token = &token_v[scanner_token_count];
        log::trace!("    parse_tuple_expr; scanner_token_count: {}, next_token: {:?}", scanner_token_count, next_token);
        match next_token.kind() {
            scanner::TokenKind::EndOfInput => {
                // TODO: More context, line number, etc.
                anyhow::bail!("Expected ')' but got EndOfInput");
            }

            scanner::TokenKind::CloseParen => {
                // This gracefully ends TupleExpr.
                // Consume the CloseParam.
                scanner_token_count += 1;
                return Ok((TupleExpr::from(expr_v), ParseStats { scanner_token_count }))
            }

            scanner::TokenKind::Comma => {
                // Consume the Comma
                scanner_token_count += 1;
                // Now parse another ExprSequence -- for now, disallow trailing Comma before CloseParen.
                let (expr_sequence, parse_stats) = parse_expr_sequence(&token_v[scanner_token_count..], ExprSequenceEnd::Unrestricted)?;
                // Accumulate the results
                expr_v.push(expr_sequence.into());
                scanner_token_count += parse_stats.scanner_token_count;
            }

            scanner::TokenKind::OpenParen |
            scanner::TokenKind::CIdentifier |
            scanner::TokenKind::DecimalLiteral |
            scanner::TokenKind::IntegerLiteral |
            scanner::TokenKind::AsciiStringLiteral => {
                let (expr_sequence, parse_stats) = parse_expr_sequence(&token_v[scanner_token_count..], ExprSequenceEnd::Unrestricted)?;
                // Accumulate the results
                expr_v.push(expr_sequence.into());
                scanner_token_count += parse_stats.scanner_token_count;
            }

            scanner::TokenKind::Whitespace => {
                panic!("This shouldn't be possible.");
            }

            scanner::TokenKind::UnrecognizedInput => {
                // TODO: Context, line number, etc.
                anyhow::bail!("unrecognized input");
            }
        }
    }
}
