use crate::Result;

mod token;

pub use crate::scanner::token::{
    AsciiStringLiteral, CIdentifier, DecimalPointLiteral, IntegerLiteral, Token, TokenKind,
    UnrecognizedInput, Whitespace,
};

pub struct ScanStats {
    pub len: usize,
    pub newline_count: usize,
}

impl ScanStats {
    pub fn from(matched_input: &str) -> Self {
        Self {
            len: matched_input.len(),
            newline_count: matched_input.matches('\n').count(),
        }
    }
}

/// Note that TokenKind::UnrecognizedInput must not be passed as token_kind (it's not a well-defined
/// operation relative to this implementation); it will panic.
pub fn try_scanning<'a>(token_kind: TokenKind, input: &'a str) -> Option<(Token<'a>, ScanStats)> {
    if token_kind == TokenKind::UnrecognizedInput {
        panic!("TokenKind::UnrecognizedInput is not a valid parameter to this function");
    }
    TOKEN_KIND_REGEX_M[token_kind]
        .find(input)
        .map(|matched_input| {
            log::trace!(
                "scanner found {:?} having matched input {:?}",
                token_kind,
                matched_input.as_str()
            );
            (
                Token::new(token_kind, matched_input.as_str()),
                ScanStats::from(matched_input.as_str()),
            )
        })
}

fn get_leading_current_input_as_string_literal(current_input: &str) -> String {
    if current_input.len() > 20 {
        format!("{:?}<truncated>", &current_input[..20])
    } else {
        format!("{:?}", &current_input)
    }
}

pub fn scan_token<'a>(input: &'a str) -> (Token<'a>, ScanStats) {
    for (token_kind, _) in TOKEN_KIND_REGEX_M.iter() {
        if token_kind == TokenKind::UnrecognizedInput {
            // UnrecognizedInput is not a valid token_kind for try_scanning; so skip it.  It's handled below.
            continue;
        }
        if let Some(x) = try_scanning(token_kind, input) {
            return x;
        }
    }
    // Handle UnrecognizedInput as the fallthrough.
    assert!(
        input.len() > 0,
        "try_scanning_end_of_input already being called should have guaranteed this condition"
    );
    let unrecognized_input_str = &input[0..1];
    (
        UnrecognizedInput::from(unrecognized_input_str).into(),
        ScanStats::from(unrecognized_input_str),
    )
}

pub fn scan<'a>(input: &'a str) -> Result<Vec<Token<'a>>> {
    log::trace!("scan; input: {:?}", input);

    let mut line_number = 1usize;
    let mut cursor = 0usize;
    let mut token_v = Vec::new();
    let mut open_paren_count = 0usize;
    loop {
        let current_input = &input[cursor..];
        log::trace!(
            "current_input: {}",
            get_leading_current_input_as_string_literal(current_input)
        );
        let (token, scan_stats) = scan_token(current_input);
        // Special handling for different tokens.
        match token {
            Token::EndOfInput => {
                // No need to record this token or update anything more.  Just return.
                return Ok(token_v);
            }
            // TODO: Maybe let the parser handle this?  Or is it good that this does some convenient extra work?
            Token::OpenParen => {
                open_paren_count += 1;
            }
            Token::CloseParen => {
                anyhow::ensure!(
                    open_paren_count > 0,
                    "unmatched ')' on line {}",
                    line_number
                );
                open_paren_count -= 1;
            }
            Token::UnrecognizedInput(_) => {
                anyhow::bail!(
                    "unrecognized input on line {}: {}",
                    line_number,
                    get_leading_current_input_as_string_literal(current_input)
                );
            }
            _ => {
                // No special handling needed.
            }
        }
        // Add the token if it's not to be ignored.
        match token {
            // Ignore token
            Token::Whitespace(_) => {}
            _ => {
                token_v.push(token);
            }
        }
        // Update line_number and cursor.
        line_number += scan_stats.newline_count;
        cursor += scan_stats.len;
    }
}

// Pre-compiled regexes for scanning Tokens.
lazy_static::lazy_static! {
    // TODO: Try using concat!(r#"^"("#, r"[ -!]", "|", etc) with newlines to visually break things up and
    // allow comments to be interspersed.
    static ref TOKEN_KIND_REGEX_M: enum_map::EnumMap<TokenKind, regex::Regex> = enum_map::enum_map! {
        TokenKind::EndOfInput => regex::Regex::new(r"^$").unwrap(),
        TokenKind::Comma => regex::Regex::new(r"^,").unwrap(),
        TokenKind::OpenParen => regex::Regex::new(r"^\(").unwrap(),
        TokenKind::CloseParen => regex::Regex::new(r"^\)").unwrap(),
        // Space, tab, or newline.
        TokenKind::Whitespace => regex::Regex::new(r"^[ \t\n]+").unwrap(),
        // C-style identifier
        TokenKind::CIdentifier => regex::Regex::new(r"^[A-Za-z_][A-Za-z_0-9]*").unwrap(),
        // An optional sign, then a string of least 1 digit containing a single decimal point, then optional
        // integer-valued exponent.
        TokenKind::DecimalPointLiteral => regex::Regex::new(r"^[+\-]?([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([eE][+\-]?(0|[1-9][0-9]*))?").unwrap(),
        // An optional sign, then either 0 or a string of digits not starting with 0.
        TokenKind::IntegerLiteral => regex::Regex::new(r"^[+\-]?(0|[1-9][0-9]*)").unwrap(),
        // An ASCII string literal consists of printable chars and escaped chars.
        // Breakdown:
        // -    [ -!] is the range from ' ' to !
        // -    Skip '"'
        // -    [#-[] is the range from # to [
        // -    Skip '\\'
        // -    [\]-~] is the range from ] to ~
        // -    \\[0abtnvfr"\\] is a single-escaped-char code (e.g. \n or \" or \\)
        // -    \\x[0-9A-Fa-f]{2} is a hex-escaped-char code, which necessarily has two hex digits (e.g. \x7F)
        // -    The *? syntax indicates an "ungreedy" match (see https://docs.rs/regex/latest/regex/#repetitions).
        // TODO: Try using concat!(r#"^"("#, r"[ -!]", "|", etc) with newlines to visually break things up and
        // allow comments to be interspersed.
        TokenKind::AsciiStringLiteral => regex::Regex::new(r#"^"([ -!]|[#-[]|[\]-~]|\\[0abtnvfr"\\]|\\x[0-9A-Fa-f]{2})*""#).unwrap(),
        // This regex is just a stand-in.  It should probably never actually be used.  But theoretically
        // it would be defined as a regex that matches a single-char complement of all of the above regexes.
        TokenKind::UnrecognizedInput => regex::Regex::new("^([\0-\x1F]|\x7F)").unwrap(),
    };
}
