mod env;
mod eval;
mod lexer;
mod object;
mod parser;
mod repl;

use repl::repl_execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl_execute()
}
