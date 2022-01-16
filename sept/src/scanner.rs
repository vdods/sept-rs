use crate::Result;

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct Whitespace<'a>(&'a str);

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct CIdentifier<'a>(&'a str);

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct DecimalLiteral<'a>(&'a str);

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct IntegerLiteral<'a>(&'a str);

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct AsciiStringLiteral<'a>(&'a str);

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq)]
pub struct UnrecognizedInput<'a>(&'a str);

// TODO: Could include line number, and maybe char range within line
/// Note that these have to be in order that the matches will be tried in.
/// Start with the quick/simple ones, then go to the slow/complex ones.  It's important
/// that try_scanning_decimal_literal happens before try_scanning_integer_literal.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    EndOfInput,
    OpenParen,
    CloseParen,
    Comma,
    Whitespace(Whitespace<'a>),
    CIdentifier(CIdentifier<'a>),
    DecimalLiteral(DecimalLiteral<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
    AsciiStringLiteral(AsciiStringLiteral<'a>),
    UnrecognizedInput(UnrecognizedInput<'a>),
}

// TODO: Derive these
impl<'a> From<Whitespace<'a>> for Token<'a> {
    fn from(whitespace: Whitespace<'a>) -> Self {
        Token::Whitespace(whitespace)
    }
}

// TODO: Derive these
impl<'a> From<CIdentifier<'a>> for Token<'a> {
    fn from(c_identifier: CIdentifier<'a>) -> Self {
        Token::CIdentifier(c_identifier)
    }
}

// TODO: Derive these
impl<'a> From<DecimalLiteral<'a>> for Token<'a> {
    fn from(decimal_literal: DecimalLiteral<'a>) -> Self {
        Token::DecimalLiteral(decimal_literal)
    }
}

// TODO: Derive these
impl<'a> From<IntegerLiteral<'a>> for Token<'a> {
    fn from(integer_literal: IntegerLiteral<'a>) -> Self {
        Token::IntegerLiteral(integer_literal)
    }
}

// TODO: Derive these
impl<'a> From<AsciiStringLiteral<'a>> for Token<'a> {
    fn from(ascii_string_literal: AsciiStringLiteral<'a>) -> Self {
        Token::AsciiStringLiteral(ascii_string_literal)
    }
}

// TODO: Derive these
impl<'a> From<UnrecognizedInput<'a>> for Token<'a> {
    fn from(unrecognized_input: UnrecognizedInput<'a>) -> Self {
        Token::UnrecognizedInput(unrecognized_input)
    }
}

impl<'a> Token<'a> {
    pub fn new(token_kind: TokenKind, input: &'a str) -> Self {
        match token_kind {
            TokenKind::EndOfInput => Token::EndOfInput,
            TokenKind::OpenParen => Token::OpenParen,
            TokenKind::CloseParen => Token::CloseParen,
            TokenKind::Comma => Token::Comma,
            TokenKind::Whitespace => Token::Whitespace(Whitespace(input)),
            TokenKind::CIdentifier => Token::CIdentifier(CIdentifier(input)),
            TokenKind::DecimalLiteral => Token::DecimalLiteral(DecimalLiteral(input)),
            TokenKind::IntegerLiteral => Token::IntegerLiteral(IntegerLiteral(input)),
            TokenKind::AsciiStringLiteral => Token::AsciiStringLiteral(AsciiStringLiteral(input)),
            TokenKind::UnrecognizedInput => Token::UnrecognizedInput(UnrecognizedInput(input)),
        }
    }
    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
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

pub struct ScanStats {
    pub len: usize,
    pub newline_count: usize,
}

impl ScanStats {
    pub fn from(matched_input: &str) -> Self {
        Self { len: matched_input.len(), newline_count: matched_input.matches('\n').count() }
    }
}

/// Note that TokenKind::UnrecognizedInput must not be passed as token_kind (it's not a well-defined
/// operation relative to this implementation); it will panic.
pub fn try_scanning<'a>(token_kind: TokenKind, input: &'a str) -> Option<(Token<'a>, ScanStats)> {
    if token_kind == TokenKind::UnrecognizedInput {
        panic!("TokenKind::UnrecognizedInput is not a valid parameter to this function");
    }
    TOKEN_KIND_REGEX_M[token_kind].find(input).map(
        |matched_input| {
            log::trace!("scanner found {:?} having matched input {:?}", token_kind, matched_input.as_str());
            (Token::new(token_kind, matched_input.as_str()), ScanStats::from(matched_input.as_str()))
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
    assert!(input.len() > 0, "try_scanning_end_of_input already being called should have guaranteed this condition");
    let unrecognized_input = &input[0..1];
    (UnrecognizedInput(unrecognized_input).into(), ScanStats::from(unrecognized_input))
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
        let (token, scan_stats) = scan_token(current_input);
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
        // This regex is just a stand-in.  It should probably never actually be used.  But theoretically
        // it would be defined as a regex that matches a single-char complement of all of the above regexes.
        TokenKind::UnrecognizedInput => regex::Regex::new("^([\0-\x1F]|\x7F)").unwrap(),
    };
}
