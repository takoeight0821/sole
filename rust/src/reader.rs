use chumsky::{prelude::*, text::Character, Error};

pub fn read(source: String) -> Vec<Spanned<Token>> {
    lexer().parse(source).unwrap()
}

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Int(String),
    String(String),
    Ident(String),
    Ctrl(char),
}

const CTRL_CHARS: &str = "[]{}()'`~^@";

pub fn ident<C: Character, E: Error<C>>() -> impl Parser<C, C::Collection, Error = E> + Copy + Clone
{
    filter(|c: &C| !(CTRL_CHARS.contains(c.to_char())) && !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect()
}

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    let int = text::int(10).map(Token::Int);

    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String);

    let ctrl = one_of(CTRL_CHARS).map(Token::Ctrl);

    let ident = ident().map(Token::Ident);

    let token = int
        .or(str_)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just(";").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}
