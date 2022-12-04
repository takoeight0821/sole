use chumsky::{prelude::*, text::Character, Error, Stream};

use crate::types::{Form, FormKind, Spanned, Token};

pub fn read(source: &str) -> Result<Form, Vec<Simple<String>>> {
    match lexer().parse(source) {
        Ok(tokens) => {
            let len = source.chars().count();
            form_parser()
                .then_ignore(end())
                .parse(Stream::from_iter(len..len + 1, tokens.into_iter()))
                .map_err(|parse_errs| {
                    parse_errs
                        .into_iter()
                        .map(|e| e.map(|tok| tok.to_string()))
                        .collect::<Vec<_>>()
                })
        }
        Err(lex_errs) => Err(lex_errs
            .into_iter()
            .map(|e| e.map(|c| c.to_string()))
            .collect()),
    }
}

const CTRL_CHARS: &str = "[]{}()'`~^@";

pub fn symbol<C: Character, E: Error<C>>() -> impl Parser<C, C::Collection, Error = E> + Copy + Clone
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

    let symbol = symbol().map(Token::Symbol);

    let token = int
        .or(str_)
        .or(ctrl)
        .or(symbol)
        .recover_with(skip_then_retry_until([]));

    let comment = just(";").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .then_ignore(end())
}

pub fn form_parser() -> impl Parser<Token, Form, Error = Simple<Token>> + Clone {
    recursive(|form| {
        let atom = select! {
            Token::Symbol(x) => FormKind::Symbol(x),
            Token::Int(x) => FormKind::Int(x.parse().unwrap()),
            Token::String(x) => FormKind::String(x),
        }
        .labelled("atom")
        .map_with_span(|kind, span| (kind, span));
        let list = form
            .repeated()
            .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
            .map(FormKind::List)
            .labelled("list")
            .map_with_span(|kind, span| (kind, span));
        atom.or(list)
    })
}
