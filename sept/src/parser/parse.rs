use crate::{dy, parser::{Expr, ExprSequence, Terminal, TupleExpr}, Result, scanner};

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
        scanner::TokenKind::DecimalPointLiteral |
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
            scanner::TokenKind::DecimalPointLiteral |
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
                // TODO: This probably can't use tail recursion optimization, so it's not going to
                // work for long tuple exprs.
                let (expr_sequence, parse_stats) = parse_expr_sequence(&token_v[scanner_token_count..], ExprSequenceEnd::Unrestricted)?;
                // Accumulate the results
                expr_v.push(expr_sequence.into());
                scanner_token_count += parse_stats.scanner_token_count;
            }

            scanner::TokenKind::OpenParen |
            scanner::TokenKind::CIdentifier |
            scanner::TokenKind::DecimalPointLiteral |
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

fn parse_value_from_terminal<'a>(terminal: &Terminal<'a>) -> Result<dy::Value> {
    match terminal.as_ref() {
        scanner::Token::EndOfInput |
        scanner::Token::OpenParen |
        scanner::Token::CloseParen |
        scanner::Token::Comma |
        scanner::Token::Whitespace(_) |
        scanner::Token::UnrecognizedInput(_) => {
            panic!("programmer error: this scanner::Token should not have made it out of parse_expr_sequence");
        }
        scanner::Token::CIdentifier(c_identifier) => {
            // NOTE: This distinction between Bool(true) and True (and similar for Bool(false) and False)
            // may not be the right way to go.
            if c_identifier.as_str() == "true" {
                Ok(dy::Value::from(true)) // This is probably a TEMP HACK
            } else if c_identifier.as_str() == "false" {
                Ok(dy::Value::from(false)) // This is probably a TEMP HACK
            } else {
                // Check if there's a non-parametric term with the given name registered with the runtime.
                match dy::RUNTIME_LA.read().unwrap().non_parametric_term(&c_identifier) {
                    Ok(non_parametric_term_value) => Ok(non_parametric_term_value),
                    Err(_) => {
                        // Otherwise create a GlobalSymRefTerm of the given identifier.
                        // TODO: Maybe check for known-at-compile-time identifiers (e.g. all the built-in
                        // types like Void and Tuple, etc) and construct those directly, instead of relying
                        // on having entries for each of those in the global symbol table.
                        Ok(dy::GlobalSymRefTerm::new_checked(c_identifier.to_string())?.into())
                    }
                }
            }
        }
        scanner::Token::DecimalPointLiteral(decimal_point_literal) => {
            // TEMP HACK -- just use f64 for now, but this isn't a generic solution.
            Ok(decimal_point_literal.parse::<f64>()?.into())
        }
        scanner::Token::IntegerLiteral(integer_literal) => {
            // TEMP HACK -- just use 64 bit ints for now, but this isn't a generic solution.
            if integer_literal.starts_with('-') || integer_literal.starts_with('+') {
                // Signed; parse as i64
                Ok(integer_literal.parse::<i64>()?.into())
            } else {
                // Unsigned; parse as u64
                Ok(integer_literal.parse::<u64>()?.into())
            }
        }
        scanner::Token::AsciiStringLiteral(ascii_string_literal) => {
            // TODO: have to un-escape the string here.
            Ok(String::from(ascii_string_literal.unescaped()?).into())
        }
    }
}

fn parse_value_from_expr<'a>(expr: &Expr<'a>) -> Result<dy::Value> {
    let value = match expr {
        Expr::Terminal(terminal) => parse_value_from_terminal(terminal)?,
        Expr::ExprSequence(expr_sequence) => parse_value_from_expr_sequence(expr_sequence)?,
        Expr::TupleExpr(_tuple_expr) => { anyhow::bail!("parse error: can't have a raw syntactical tuple; must construct it by prefixing the raw syntactical tuple with `Tuple` (this is just a choice to be totally formal about this parser in order to keep it as simple as possible, at a small cost of usability)"); }
    };
    Ok(value)
}

fn parse_tuple_term_from_tuple_expr<'a>(tuple_expr: &TupleExpr<'a>) -> Result<dy::TupleTerm> {
    let element_v =
        tuple_expr
            .as_ref()
            .iter()
            .map(|expr| parse_value_from_expr(expr))
            .collect::<Result<Vec<dy::Value>>>()?;
    Ok(dy::TupleTerm::from(element_v))
}

fn parse_tuple_term_from_expr<'a>(expr: &Expr<'a>) -> Result<dy::TupleTerm> {
    let tuple_expr =
        expr.as_tuple_expr()
            .ok_or_else(
                || anyhow::anyhow!("parse error: expected TupleExpr, but got {:?}", expr.kind())
            )?;
    Ok(parse_tuple_term_from_tuple_expr(tuple_expr)?)
}

// fn parse_value_from_expr_sequence<'a>(expr_v: &[Expr<'a>]) -> Result<dy::Value> {
fn parse_value_from_expr_sequence<'a>(expr_sequence: &ExprSequence<'a>) -> Result<dy::Value> {
    // The expr_sequence must be a Expr::Terminal followed by 0 or more instances of Expr::TupleExpr.
    // TODO: Figure out if an empty sequence has a canonical meaning as a dy::Value.
//     anyhow::ensure!(expr_v.len() > 0, "parse error: empty input");
    anyhow::ensure!(expr_sequence.len() > 0, "parse error: empty input");
    // Inductive definition.  The expr sequence must be [terminal, tuple_expr_1, ..., tuple_expr_n]
    // for some n (which could be 0).  First, extract the terminal and parse it.  This is the "head
    // value".  If n == 0, then this value is returned directly.  If n > 0, then the head value is
    // the constructor and the next tuple is fed in as its parameters to construct a value into
    // which the next tuple is fed in as its constructor parameters, etc.
//     let head_value = match &expr_v[0] {
    let head_value = match &expr_sequence[0] {
        Expr::Terminal(terminal) => parse_value_from_terminal(terminal)?,
        Expr::ExprSequence(_) => { panic!("programmer error; it should not be possible to compose ExprSequence directly within ExprSequence"); }
        // TODO: Figure out how to notate the location of the error in the input.
        Expr::TupleExpr(_tuple_expr) => { anyhow::bail!("parse error: expected expr sequence to start with a terminal, but found a tuple expr"); }
    };
    log::debug!("head_value: {:?}", head_value);
    let mut value = head_value;
    for i in 1..expr_sequence.len() {
        let parameter_t = parse_tuple_term_from_expr(&expr_sequence[i])?;
        use dy::Constructor;
        log::debug!("constructing using value: {:?} and parameter_t: {:?}", value, parameter_t);
        value = value.construct(parameter_t)?;
    }
    Ok(value)
}

// TODO: Make a generic version of this that returns a particular type
fn parse_value_from_tokens<'a>(token_v: &[scanner::Token<'a>]) -> Result<dy::Value> {
    let (expr_sequence, _parse_stats) = parse_expr_sequence(token_v, ExprSequenceEnd::OnlyOnEndOfInput)?;
    log::debug!("parse_value_from_tokens\n\ttoken_v: {:#?}\n\texpr_sequence: {:#?}", token_v, expr_sequence);
//     Ok(parse_value_from_expr_sequence(expr_sequence.as_slice().iter().collect::<Vec<_>>().as_slice())?)
    Ok(parse_value_from_expr_sequence(&expr_sequence)?)
}

pub fn parse_value(input: &str) -> Result<dy::Value> {
    let token_v = scanner::scan(input).expect("test");
    Ok(parse_value_from_tokens(&token_v)?)
}

/// This parses a string and turns it into a `dy::Deconstruction`, which is the "raw" syntactical
/// representation of what was parsed, and its `reconstruct` or `reconstruction` methods can be used
/// to attempt to assemble it into a `dy::Value` (even if there are no syntax errors in the string
/// input, the reconstruction may still fail because of semantic errors).
pub fn parse_deconstruction(input: &str) -> Result<dy::Deconstruction> {
    let token_v = scanner::scan(input).expect("test");
    Ok(parse_deconstruction_from_tokens(&token_v)?)
}

fn parse_deconstruction_from_tokens<'a>(token_v: &[scanner::Token<'a>]) -> Result<dy::Deconstruction> {
    let (expr_sequence, _parse_stats) = parse_expr_sequence(&token_v, ExprSequenceEnd::OnlyOnEndOfInput)?;
    // We expect a Terminal, then at least 0 TupleExpr-s.
    let expr_v: Vec<_> = expr_sequence.into();
    anyhow::ensure!(!expr_v.is_empty(), "no expressions");
    Ok(parse_deconstruction_impl(&expr_v)?)
}

fn parse_deconstruction_impl<'a>(expr_v: &[Expr<'a>]) -> Result<dy::Deconstruction> {
//     use std::ops::Deref;
    match expr_v.len() {
        0 => { anyhow::bail!("empty expr_v"); }
        1 => {
            // Base case
            match expr_v.first().unwrap() {
                Expr::Terminal(terminal) => {
                    Ok(dy::Deconstruction::NonParametric(
                        dy::NonParametricDeconstruction::new(
                            parse_value_from_terminal(terminal)?
                        )?
                    ))
                }
                Expr::TupleExpr(tuple_expr) => {
                    anyhow::bail!("expected CIdentifier but got {:?}", tuple_expr);
                }
                Expr::ExprSequence(expr_sequence) => {
                    Ok(parse_deconstruction_impl(&expr_sequence.as_ref()[..])?)
                }
            }
        }
        _ => {
            // Inductive case.  Last term is the parameters, all but last form the constructor.
            let parameter_dv = match expr_v.last().unwrap() {
                Expr::TupleExpr(tuple_expr) => {
                    let mut parameter_dv = Vec::with_capacity(tuple_expr.as_ref().len());
                    for i in 0..tuple_expr.as_ref().len() {
                        parameter_dv.push(parse_deconstruction_impl(&tuple_expr.as_ref()[i..i+1])?);
                    }
                    parameter_dv
                }
                Expr::Terminal(terminal) => {
                    anyhow::bail!("expected TupleExpr but got {:?}", terminal);
                }
                Expr::ExprSequence(expr_sequence) => {
                    anyhow::bail!("expected TupleExpr but got {:?}", expr_sequence);
                }
            };
            Ok(dy::Deconstruction::from(
                dy::ParametricDeconstruction::new(
                    // constructor comes from all but last.
                    parse_deconstruction_impl(&expr_v[..expr_v.len()-1])?,
                    // parameters are last.
                    parameter_dv,
                )
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This will run once at load time (i.e. presumably before main function is called).
    #[ctor::ctor]
    fn overall_init() {
        env_logger::try_init().unwrap();
    }

    fn test_parse_expr_sequence_case(input: &str) {
        log::debug!("input: {:?}", input);
        let token_v = scanner::scan(input).expect("test");
        log::debug!("token_v ({} elements): {:?}", token_v.len(), token_v);
        let (expr_sequence, parse_stats) = parse_expr_sequence(&token_v, ExprSequenceEnd::OnlyOnEndOfInput).expect("test");
        log::debug!("expr_sequence:\n{:#?}", expr_sequence);
        log::debug!("parse_stats: {:?}", parse_stats);
        assert_eq!(parse_stats.scanner_token_count, token_v.len());
    }

    #[test]
    #[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
    fn test_parse_expr_sequence() {
        // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
        dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();
        dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().define_symbol("weewoo", true.into()).expect("test");

        test_parse_expr_sequence_case("Tuple");
        test_parse_expr_sequence_case("123");
        test_parse_expr_sequence_case("Float64 34.002e10");
        test_parse_expr_sequence_case("(Float64 34.002e10)");
        test_parse_expr_sequence_case(" (Float64, 34.002e10) ");
        test_parse_expr_sequence_case(" (Float64, 34.002e10 ()) ");
        test_parse_expr_sequence_case("Tuple(Sint8(123), Bool(true), Void, Utf8String)");
        test_parse_expr_sequence_case("ArrayES(Float32, 4)(100.0, 8.9, 0.0, 1.0)");
        test_parse_expr_sequence_case("GlobalSymRef(Utf8String(\"weewoo\"))");
    }
}
