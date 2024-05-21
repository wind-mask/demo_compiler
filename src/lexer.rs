use std::fmt;

use logos::Lexer;
use logos::Logos;

use logos::Skip;
use logos::SpannedIter;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer_<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token<'input>>,
}
impl<'input> Lexer_<'input> {
    pub fn new(input: &'input str) -> Self {
        let token_stream = Token::lexer(input).spanned();
        Self { token_stream }
    }
}

impl<'input> Iterator for Lexer_<'input> {
    type Item = Spanned<Token<'input>, usize, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut in_block_comment = false;
        while let Some((Ok(t), s)) = self.token_stream.next() {
            match t {
                Token::StartBlockComment => {
                    in_block_comment = true;
                }
                Token::EndBlockComment => {
                    in_block_comment = false;
                }
                _ => {
                    if !in_block_comment {
                        return Some(Ok((s.start, t, s.end)));
                    }
                }
            };
        }
        match self.token_stream.next() {
            Some((Err(e), _)) => Some(Err(e)),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token<'input> {
    // #[token("\"", priority = 201)]
    // Quotes,
    // #[token("\\", priority = 201)]
    // Backslash,
    #[regex(r"//[^\n]*", |_|Skip, priority = 100)]
    #[regex(r"\s+", |_|Skip, priority = 100)]
    CommentOrWhitespace,
    // Start of a block comment.
    #[token("/*")]
    StartBlockComment,
    // End of a block comment.
    #[token("*/")]
    EndBlockComment,
    #[token("extern")]
    Extern,
    #[token("use")]
    Use,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("fn")]
    Fn,
    #[token("->")]
    Arrow,
    // #[token("_",priority=2)]
    // Underscore,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    // #[token("usize")]
    // Usize,
    // #[token("f64")]
    // F64,
    // #[token("i64")]
    // I64,
    // #[token("bool")]
    // Bool,
    #[token("const")]
    Const,
    #[regex(r"[-]?\d+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
    #[token("nan", ignore(ascii_case))]
    Nan,
    #[token("inf", ignore(ascii_case))]
    Inf,
    #[regex(r"-?(\d+\.\d*|\d*\.\d+)([eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[token("==")]
    Eq,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("::")]
    DoubleColon,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Xor,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("&")]
    BitAnd,
    #[token("||")]
    Or,
    #[token("|")]
    BitOr,
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    StarAssign,
    #[token("/=")]
    SlashAssign,
    #[token("%=")]
    PercentAssign,
    #[token("^=")]
    XorAssign,
    #[token("&=")]
    BitAndAssign,
    #[token("|=")]
    BitOrAssign,
    #[token("&&=")]
    AndAssign,
    #[token("||=")]
    OrAssign,
    #[regex("[\\p{L}_\\u{2600}-\\u{26FF}\\u{1F300}-\\u{1F5FF}\\u{1F600}-\\u{1F64F}\\u{1F680}-\\u{1F6FF}\\u{1F900}-\\u{1F9FF}\\u{1FA00}-\\u{1FA6F}][\\p{L}\\p{N}_\\u{2600}-\\u{26FF}\\u{1F300}-\\u{1F5FF}\\u{1F600}-\\u{1F64F}\\u{1F680}-\\u{1F6FF}\\u{1F900}-\\u{1F9FF}\\u{1FA00}-\\u{1FA6F}]*", priority = 1 , callback = |lex|lex.slice())]
    Identifier(&'input str),
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\r|\\\"|\\\\)*""#,callback = escape_string)]
    StringLiteral(String),
}
fn escape_string<'input>(lex: &mut Lexer<'input, Token<'input>>) -> String {
    let mut s = lex.slice();
    s = &s[1..s.len() - 1];
    s.replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
#[test]
fn test_token() {
    use std::fs;
    let path = "R:\\CODE\\Compile_demo\\demo_compiler\\src\\test\\src.dsrc";
    let input = fs::read_to_string(path).unwrap();
    let l = Lexer_::new(&input);
    for x in l {
        println!("{:?}", x);
    }
}
