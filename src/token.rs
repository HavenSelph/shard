use crate::location::Span;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenKind {
    Ampersand,
    At,
    Backslash,
    Bang,
    Caret,
    CharLiteral,
    Colon,
    Comma,
    Dollar,
    Dot,
    DoubleQuote,
    EOF,
    Equals,
    FatArrow,
    FloatLiteral,
    GreaterThan,
    GreaterThanEquals,
    Identifier,
    IntegerLiteral,
    Jmp,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanEquals,
    Minus,
    MinusMinus,
    Newline,
    NotEquals,
    Percent,
    Pipe,
    Plus,
    PlusPlus,
    Pound,
    Question,
    Register,
    Ret,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
    SingleQuote,
    Slash,
    Star,
    StringLiteral,
    Tilde,
    TinyArrowLeft,
    TinyArrowRight,
    Underscore,
}

// pub enum RegSize {
//     ByteHigh,
//     ByteLow,
//     Word,
//     DoubleWord,
//     QuadWord,
//     Arch,  // architecture dependent
// }



pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
    pub flag: u8,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, text: String) -> Token {
        Token {
            kind,
            span,
            text,
            flag: 0,
        }
    }

    pub fn new_simple(kind: TokenKind, span: Span) -> Token {
        Token::new(kind, span, String::new())
    }

    pub fn new_eof(span: Span) -> Token {
        Token::new_simple(TokenKind::EOF, span)
    }

    pub fn from_string(span: Span, text: String) -> Token {
        Token {
            kind: match text.as_ref() {
                "ret" => TokenKind::Ret,
                "jmp" => TokenKind::Jmp,
                _ => TokenKind::Identifier,
            },
            span,
            text,
            flag: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)?;
        if !self.text.is_empty() {
            if self.flag != 0 {
                write!(f, "({:?}, f{})", self.text, self.flag)?;
            } else {
                write!(f, "({:?})", self.text)?;
            }
        }
        Ok(())
    }
}
