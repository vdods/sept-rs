use crate::Result;

// TODO: Could include line number, and maybe char range within line
/// Note that these have to be in order that the matches will be tried in.
/// Start with the quick/simple ones, then go to the slow/complex ones.  It's important
/// that try_scanning_decimal_literal happens before try_scanning_integer_literal.
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    EndOfInput,
    OpenParen,
    CloseParen,
    Comma,
    Whitespace(&'a str),
    CIdentifier(&'a str),
    DecimalLiteral(&'a str),
    IntegerLiteral(&'a str),
    AsciiStringLiteral(&'a str),
    UnrecognizedInput(&'a str),
}

impl<'a> Token<'a> {
    pub fn new(token_kind: TokenKind, input: &'a str) -> Self {
        match token_kind {
            TokenKind::EndOfInput => Token::EndOfInput,
            TokenKind::OpenParen => Token::OpenParen,
            TokenKind::CloseParen => Token::CloseParen,
            TokenKind::Comma => Token::Comma,
            TokenKind::Whitespace => Token::Whitespace(input),
            TokenKind::CIdentifier => Token::CIdentifier(input),
            TokenKind::DecimalLiteral => Token::DecimalLiteral(input),
            TokenKind::IntegerLiteral => Token::IntegerLiteral(input),
            TokenKind::AsciiStringLiteral => Token::AsciiStringLiteral(input),
            TokenKind::UnrecognizedInput => Token::UnrecognizedInput(input),
        }
    }
}

// TODO: Derive this somehow
#[derive(Clone, Copy, Debug, enum_map::Enum, PartialEq)]
pub enum TokenKind {
    EndOfInput,
    OpenParen,
    CloseParen,
    Comma,
    Whitespace,
    CIdentifier,
    DecimalLiteral,
    IntegerLiteral,
    AsciiStringLiteral,
    UnrecognizedInput,
}

impl<'a> From<&Token<'a>> for TokenKind {
    fn from(token: &Token<'a>) -> Self {
        match token {
            Token::EndOfInput => TokenKind::EndOfInput,
            Token::OpenParen => TokenKind::OpenParen,
            Token::CloseParen => TokenKind::CloseParen,
            Token::Comma => TokenKind::Comma,
            Token::Whitespace(_) => TokenKind::Whitespace,
            Token::CIdentifier(_) => TokenKind::CIdentifier,
            Token::DecimalLiteral(_) => TokenKind::DecimalLiteral,
            Token::IntegerLiteral(_) => TokenKind::IntegerLiteral,
            Token::AsciiStringLiteral(_) => TokenKind::AsciiStringLiteral,
            Token::UnrecognizedInput(_) => TokenKind::UnrecognizedInput,
        }
    }
}

pub struct MatchStats {
    pub len: usize,
    pub newline_count: usize,
}

impl MatchStats {
    pub fn from(matched_input: &str) -> Self {
        Self { len: matched_input.len(), newline_count: matched_input.matches('\n').count() }
    }
}

pub fn try_scanning<'a>(token_kind: TokenKind, input: &'a str) -> Option<(Token<'a>, MatchStats)> {
    TOKEN_KIND_REGEX_M[token_kind].find(input).map(
        |matched_input| {
            log::trace!("scanner found {:?} having matched input {:?}", token_kind, matched_input.as_str());
            (Token::new(token_kind, matched_input.as_str()), MatchStats::from(matched_input.as_str()))
        }
    )
}

fn get_leading_current_input_as_string_literal(current_input: &str) -> String {
    if current_input.len() > 20 {
        format!("{:?}<truncated>", &current_input[..20])
    } else {
        format!("{:?}", &current_input)
    }
}

pub fn scan_token<'a>(input: &'a str) -> (Token<'a>, MatchStats) {
    for (token_kind, _) in TOKEN_KIND_REGEX_M.iter() {
        if let Some(x) = try_scanning(token_kind, input) {
            return x;
        }
    }
    log::warn!("There is an error in the definition of UnrecognizedInput's regex vs the others, and a case has slipped through.  Returning UnrecognizedInput as a safe default");
    assert!(input.len() > 0, "try_scanning_end_of_input already being called should have guaranteed this condition");
    let unrecognized_input = &input[0..1];
    (Token::UnrecognizedInput(unrecognized_input), MatchStats::from(unrecognized_input))
}

pub fn scan<'a>(input: &'a str) -> Result<Vec<Token<'a>>> {
    log::trace!("scan; input: {:?}", input);

    let mut line_number = 1usize;
    let mut cursor = 0usize;
    let mut token_v = Vec::new();
    let mut open_paren_count = 0usize;
    loop {
        let current_input = &input[cursor..];
        log::trace!("current_input: {}", get_leading_current_input_as_string_literal(current_input));
        let (token, match_stats) = scan_token(current_input);
        // Special handling for different tokens.
        match token {
            Token::EndOfInput => {
                // No need to record this token or update anything more.  Just return.
                return Ok(token_v);
            },
            // TODO: Maybe let the parser handle this?  Or is it good that this does some convenient extra work?
            Token::OpenParen => {
                open_paren_count += 1;
            },
            Token::CloseParen => {
                anyhow::ensure!(open_paren_count > 0, "unmatched ')' on line {}", line_number);
                open_paren_count -= 1;
            },
            Token::UnrecognizedInput(_) => {
                anyhow::bail!("unrecognized input on line {}: {}", line_number, get_leading_current_input_as_string_literal(current_input));
            },
            _ => {
                // No special handling needed.
            }
        }
        // Add the token if it's not to be ignored.
        match token {
            // Ignore token
            Token::Whitespace(_) => { }
            _ => { token_v.push(token); }
        }
        // Update line_number and cursor.
        line_number += match_stats.newline_count;
        cursor += match_stats.len;
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
        TokenKind::DecimalLiteral => regex::Regex::new(r"^[+\-]?([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([eE][+\-]?(0|[1-9][0-9]*))?").unwrap(),
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
        // A single unrecognized char.
        TokenKind::UnrecognizedInput => regex::Regex::new("^([\0-\x1F]|\x7F)").unwrap(),
    };
}
