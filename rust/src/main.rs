mod reader;
mod types;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::Simple;
use reader::read;
use rustyline::{error::ReadlineError, Editor};
use types::Form;

fn eval(expr: Form) -> Form {
    expr
}

fn print(value: Form) -> String {
    format!("{:?}", value)
}

fn rep(source: String) -> String {
    match read(&source) {
        Ok(form) => print(eval(form)),
        Err(errs) => error_report(&source, errs),
    }
}

fn error_report(source: &str, errs: Vec<Simple<String>>) -> String {
    let mut buf = Vec::new();
    for e in errs {
        let report = Report::build(ReportKind::Error, (), e.span().start);
        let report = match e.reason() {
            chumsky::error::SimpleReason::Unexpected => report
                .with_message(format!(
                    "{}, expected {}",
                    if e.found().is_some() {
                        "Unexpected token in input"
                    } else {
                        "Unexpected end of input"
                    },
                    if e.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        e.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ))
                .with_label(
                    Label::new(e.span())
                        .with_message(format!(
                            "Unexpected token {}",
                            e.found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_label(
                    Label::new(span.clone())
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                )
                .with_label(
                    Label::new(e.span())
                        .with_message(format!(
                            "Must be closed before this {}",
                            e.found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                Label::new(e.span())
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            ),
        };
        report
            .finish()
            .write(Source::from(source), &mut buf)
            .unwrap();
    }
    String::from_utf8(buf).unwrap()
}

const HISTORY_PATH: &str = ".sole-history.dat";

fn main() -> rustyline::Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(HISTORY_PATH).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", rep(line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_PATH)
}
