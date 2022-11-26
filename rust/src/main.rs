use std::io::{self, Write};

use rustyline::{error::ReadlineError, Editor};

fn read(source: String) -> String {
    source
}

fn eval(expr: String) -> String {
    expr
}

fn print(value: String) -> String {
    value
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
