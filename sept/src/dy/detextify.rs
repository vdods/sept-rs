use crate::{dy, parser, Result, scanner};

pub fn detextify(input: &str) -> Result<dy::Deconstruction> {
    let token_v = scanner::scan(input)?;
    let (expr_sequence, _) = parser::parse_expr_sequence(&token_v, parser::ExprSequenceEnd::OnlyOnEndOfInput)?;
    // We expect a Terminal, then at least 0 TupleExpr-s.
    let expr_v: Vec<_> = expr_sequence.into();
    anyhow::ensure!(!expr_v.is_empty(), "no expressions");
    Ok(detextify_impl(&expr_v)?)
}

fn detextify_impl<'a>(expr_v: &[parser::Expr<'a>]) -> Result<dy::Deconstruction> {
    use std::ops::Deref;
    match expr_v.len() {
        0 => { anyhow::bail!("empty expr_v"); }
        1 => {
            // Base case
            match expr_v.first().unwrap() {
                parser::Expr::Terminal(terminal) => {
                    match terminal.as_ref() {
                        scanner::Token::CIdentifier(c_identifier) => {
                            // TEMP HACK -- Handle exceptional "keywords"
                            if *c_identifier.deref() == "true" {
                                Ok(dy::Deconstruction::NonParametric(
                                    dy::NonParametricDeconstruction::new(
                                        dy::Value::from(true)
                                    )?
                                ))
                            } else if *c_identifier.deref() == "false" {
                                Ok(dy::Deconstruction::NonParametric(
                                    dy::NonParametricDeconstruction::new(
                                        dy::Value::from(false)
                                    )?
                                ))
                            } else {
                                Ok(dy::Deconstruction::NonParametric(
                                    dy::NonParametricDeconstruction::new(
                                        dy::RUNTIME_LA.read().unwrap().non_parametric_term(&c_identifier)?
                                    )?
                                ))
                            }
                        }
                        scanner::Token::DecimalLiteral(decimal_literal) => {
                            // TEMP HACK -- just use f64 for now, but this isn't a generic solution.
                            Ok(dy::Deconstruction::NonParametric(
                                dy::NonParametricDeconstruction::new(
                                    dy::Value::from(decimal_literal.parse::<f64>()?)
                                )?
                            ))
                        }
                        scanner::Token::IntegerLiteral(integer_literal) => {
                            // TEMP HACK -- just use 64 bit ints for now, but this isn't a generic solution.
                            let value = if integer_literal.starts_with('-') || integer_literal.starts_with('+') {
                                // Signed; parse as i64
                                dy::Value::from(integer_literal.parse::<i64>()?)
                            } else {
                                // Unsigned; parse as u64
                                dy::Value::from(integer_literal.parse::<u64>()?)
                            };
                            Ok(dy::Deconstruction::NonParametric(
                                dy::NonParametricDeconstruction::new(
                                    value
                                )?
                            ))
                        }
                        scanner::Token::AsciiStringLiteral(ascii_string_literal) => {
                            // TODO: have to un-escape the string here.
                            Ok(dy::Deconstruction::NonParametric(
                                dy::NonParametricDeconstruction::new(
                                    dy::Value::from(String::from(ascii_string_literal.unescaped()?))
                                )?
                            ))
                        }
                        x => {
                            anyhow::bail!("expected CIdentifier but got {:?}", x);
                        }
                    }
                }
                parser::Expr::TupleExpr(tuple_expr) => {
                    anyhow::bail!("expected CIdentifier but got {:?}", tuple_expr);
                }
                parser::Expr::ExprSequence(expr_sequence) => {
                    Ok(detextify_impl(&expr_sequence.as_ref()[..])?)
                }
            }
        }
        _ => {
            // Inductive case.  Last term is the parameters, all but last form the constructor.
            let parameter_dv = match expr_v.last().unwrap() {
                parser::Expr::TupleExpr(tuple_expr) => {
                    let mut parameter_dv = Vec::with_capacity(tuple_expr.as_ref().len());
                    for i in 0..tuple_expr.as_ref().len() {
                        parameter_dv.push(detextify_impl(&tuple_expr.as_ref()[i..i+1])?);
                    }
                    parameter_dv
                }
                parser::Expr::Terminal(terminal) => {
                    anyhow::bail!("expected TupleExpr but got {:?}", terminal);
                }
                parser::Expr::ExprSequence(expr_sequence) => {
                    anyhow::bail!("expected TupleExpr but got {:?}", expr_sequence);
                }
            };
            Ok(dy::Deconstruction::from(
                dy::ParametricDeconstruction::new(
                    // constructor comes from all but last.
                    detextify_impl(&expr_v[..expr_v.len()-1])?,
                    // parameters are last.
                    parameter_dv,
                )
            ))
        }
    }
}
