use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::{Error, KvData, Result, types::KvEntry};

#[derive(Clone, Debug, Eq, PartialEq)]
enum TokenKind<'a> {
    OpenBrace,
    CloseBrace,
    Atom(Cow<'a, str>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Token<'a> {
    kind: TokenKind<'a>,
    conditional: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub escape_sequences: bool,
    pub numeric_inference: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            escape_sequences: false,
            numeric_inference: true,
        }
    }
}

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
    peeked: Option<Token<'a>>,
    options: Options,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let input = input.strip_prefix('\u{feff}').unwrap_or(input);
        Self {
            input,
            pos: 0,
            peeked: None,
            options: Default::default(),
        }
    }

    pub fn options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }

    pub fn with_escape_sequences(input: &'a str) -> Self {
        Self::new(input).escape_sequences(true)
    }

    pub fn escape_sequences(mut self, state: bool) -> Self {
        self.options.escape_sequences = state;
        self
    }

    pub fn numeric_inference(mut self, state: bool) -> Self {
        self.options.numeric_inference = state;
        self
    }

    pub fn parse(&mut self) -> Result<KvEntry<'a>> {
        self.skip_conditionals()?;
        self.parse_entry()
    }

    fn next(&mut self) -> Result<Option<Token<'a>>> {
        if let Some(token) = self.peeked.take() {
            return Ok(Some(token));
        }

        self.read_token()
    }

    fn peek(&mut self) -> Result<Option<Token<'a>>> {
        if self.peeked.is_none() {
            self.peeked = self.read_token()?;
        }

        Ok(self.peeked.clone())
    }

    fn parse_compound(&mut self) -> Result<Vec<KvEntry<'a>>> {
        let mut entries = Vec::new();

        loop {
            let Some(token) = self.peek()? else {
                return Err(Error::UnexpectedEof);
            };

            match token.kind {
                TokenKind::CloseBrace => {
                    self.next()?;
                    break;
                }
                _ if token.conditional => {
                    self.next()?;
                }
                _ => entries.push(self.parse_entry()?),
            }
        }

        Ok(entries)
    }

    fn parse_entry(&mut self) -> Result<KvEntry<'a>> {
        let name = self.read_atom()?;
        self.skip_conditionals()?;

        let data = match self.peek()? {
            Some(Token {
                kind: TokenKind::OpenBrace,
                ..
            }) => {
                self.next()?;
                KvData::Compound(self.parse_compound()?)
            }
            _ => {
                let raw = self.read_atom()?;
                self.skip_conditionals()?;
                self.parse_value(raw)
            }
        };

        Ok(KvEntry { name, data })
    }

    fn read_atom(&mut self) -> Result<Cow<'a, str>> {
        match self.next()? {
            Some(Token {
                kind: TokenKind::Atom(s),
                conditional: false,
            }) => Ok(s),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn skip_conditionals(&mut self) -> Result<()> {
        while matches!(
            self.peek()?,
            Some(Token {
                conditional: true,
                ..
            })
        ) {
            self.next()?;
        }

        Ok(())
    }

    fn parse_value(&self, value: Cow<'a, str>) -> KvData<'a> {
        if !self.options.numeric_inference {
            return KvData::String(value);
        }

        let raw = value.as_ref();

        if raw.is_empty() {
            return KvData::String(value);
        }

        if let Some(v) = parse_uint64_hex(raw) {
            return KvData::UInt64(v);
        }

        if looks_float_like(raw)
            && let Ok(v) = raw.parse::<f32>()
        {
            return KvData::Float(v);
        }

        if let Ok(v) = raw.parse::<i32>() {
            return KvData::Int(v);
        }

        KvData::String(value)
    }

    fn read_token(&mut self) -> Result<Option<Token<'a>>> {
        self.skip_ws_and_comments();

        let Some(&byte) = self.input.as_bytes().get(self.pos) else {
            return Ok(None);
        };

        match byte {
            b'{' => {
                self.pos += 1;
                Ok(Some(Token {
                    kind: TokenKind::OpenBrace,
                    conditional: false,
                }))
            }
            b'}' => {
                self.pos += 1;
                Ok(Some(Token {
                    kind: TokenKind::CloseBrace,
                    conditional: false,
                }))
            }
            b'"' => self.read_quoted().map(Some),
            _ => Ok(Some(self.read_bare())),
        }
    }

    fn read_quoted(&mut self) -> Result<Token<'a>> {
        self.pos += 1;
        let start = self.pos;

        if !self.options.escape_sequences {
            while let Some(&byte) = self.input.as_bytes().get(self.pos) {
                if byte == b'"' {
                    let value = Cow::Borrowed(&self.input[start..self.pos]);
                    self.pos += 1;
                    return Ok(Token {
                        kind: TokenKind::Atom(value),
                        conditional: false,
                    });
                }

                self.pos += 1;
            }

            return Err(Error::UnexpectedEof);
        }

        let mut out = None::<String>;
        let mut segment_start = start;

        while let Some(&byte) = self.input.as_bytes().get(self.pos) {
            match byte {
                b'"' => {
                    let value = if let Some(mut out) = out {
                        out.push_str(&self.input[segment_start..self.pos]);
                        Cow::Owned(out)
                    } else {
                        Cow::Borrowed(&self.input[start..self.pos])
                    };
                    self.pos += 1;
                    return Ok(Token {
                        kind: TokenKind::Atom(value),
                        conditional: false,
                    });
                }
                b'\\' => {
                    // flush literal run accumulated since the previous escape
                    // before appending the decoded escape.
                    let out = out.get_or_insert_with(String::new);
                    out.push_str(&self.input[segment_start..self.pos]);
                    self.pos += 1;

                    let Some(ch) = self.input[self.pos..].chars().next() else {
                        return Err(Error::UnexpectedEof);
                    };

                    out.push(match ch {
                        'n' => '\n',
                        't' => '\t',
                        'v' => '\u{000b}',
                        'b' => '\u{0008}',
                        'r' => '\r',
                        'f' => '\u{000c}',
                        'a' => '\u{0007}',
                        '\\' => '\\',
                        '?' => '?',
                        '\'' => '\'',
                        '"' => '"',
                        other => other,
                    });
                    self.pos += ch.len_utf8();
                    segment_start = self.pos;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        Err(Error::UnexpectedEof)
    }

    fn read_bare(&mut self) -> Token<'a> {
        let start = self.pos;
        let mut conditional_start = false;
        let mut conditional = false;

        while let Some(&byte) = self.input.as_bytes().get(self.pos) {
            if byte == b'"' || byte == b'{' || byte == b'}' || byte.is_ascii_whitespace() {
                break;
            }

            if byte == b'[' {
                conditional_start = true;
            } else if byte == b']' && conditional_start {
                conditional = true;
            }

            self.pos += 1;
        }

        Token {
            kind: TokenKind::Atom(Cow::Borrowed(&self.input[start..self.pos])),
            conditional,
        }
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while self
                .input
                .as_bytes()
                .get(self.pos)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.pos += 1;
            }

            if self.input.as_bytes().get(self.pos..self.pos + 2) != Some(b"//") {
                break;
            }

            self.pos += 2;
            while self
                .input
                .as_bytes()
                .get(self.pos)
                .is_some_and(|&byte| byte != b'\n')
            {
                self.pos += 1;
            }
        }
    }
}

fn parse_uint64_hex(value: &str) -> Option<u64> {
    let digits = value.strip_prefix("0x")?;
    if digits.len() != 16 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }

    u64::from_str_radix(digits, 16).ok()
}

fn looks_float_like(value: &str) -> bool {
    value.bytes().any(|b| matches!(b, b'.' | b'e' | b'E'))
}
