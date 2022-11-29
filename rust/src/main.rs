mod reader;

use reader::{read, Spanned, Token};
use rustyline::{error::ReadlineError, Editor};

fn eval(expr: Vec<Spanned<Token>>) -> Vec<Spanned<Token>> {
    expr
}

fn print(value: Vec<Spanned<Token>>) -> String {
    format!("{:?}", value)
}

fn rep(source: String) -> String {
    print(eval(read(source)))
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
