mod parser;
mod command;

use parser::cli_words;
use command::{elements_to_args, run_cli_command};
use crate::parser::Parser;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} '<xml cli input>'", args[0]);
        return;
    }

    let input = &args[1];

    match cli_words().parse(input) {
        Ok((_rest, elements)) => {
            let cmd_args = elements_to_args(&elements);
            println!("Running command: {:?}", cmd_args);
            run_cli_command(cmd_args);
        }
        Err(err) => {
            eprintln!("Parse error at: {}", err);
        }
    }
}


