use std::fmt::Display;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Symbol(String),
    Int(String),
    String(String),
    Ctrl(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Symbol(x) => write!(f, "{}", x),
            Token::Int(x) => write!(f, "{}", x),
            Token::String(x) => write!(f, "\"{}\"", x),
            Token::Ctrl(x) => write!(f, "{}", x),
        }
    }
}

pub type Form = Spanned<FormKind>;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum FormKind {
    Symbol(String),
    Int(i64),
    String(String),
    List(Vec<Form>),
}
