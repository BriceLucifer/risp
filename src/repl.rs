
use clap::*;
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use linefeed::{Interface, ReadResult};
use crate::object::Object;
use crate::env::Env;
use crate::eval::eval;
use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;

const PROMPT: &str = "\x1b[38;2;255;165;0m🦀lisp-rs >> \x1b[0m";

pub fn repl_execute() -> Result<(), Box<dyn std::error::Error>> {
    let _mathes = App::new("Lisp")
        .version("1.0")
        .author("Your Name <github.com/BriceLucifer>")
        .about("A simple Lisp REPL written in Rust")
        .get_matches();

    let reader = Interface::new(PROMPT)?;
    let mut env = Rc::new(RefCell::new(Env::new()));

    reader.set_prompt(PROMPT)?;

    let mut stdout = stdout();

    while let ReadResult::Input(input) = reader.read_line()? {
        if input.trim() == "exit" {
            break;
        }
        let val = eval(&input, &mut env)?;
        match val {
            Object::Void => {}
            Object::Integer(n) => {
                stdout.execute(SetForegroundColor(Color::Green))?;
                println!("{}", n);
                stdout.execute(ResetColor)?;
            }
            Object::Bool(b) => {
                stdout.execute(SetForegroundColor(Color::Yellow))?;
                println!("{}", b);
                stdout.execute(ResetColor)?;
            }
            Object::Symbol(s) => {
                stdout.execute(SetForegroundColor(Color::Blue))?;
                println!("{}", s);
                stdout.execute(ResetColor)?;
            }
            Object::Lambda(params, body) => {
                stdout.execute(SetForegroundColor(Color::Magenta))?;
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
                stdout.execute(ResetColor)?;
            }
            _ => {
                stdout.execute(SetForegroundColor(Color::Cyan))?;
                println!("{}", val);
                stdout.execute(ResetColor)?;
            }
        }
    }

    stdout.execute(SetForegroundColor(Color::Red))?;
    println!("Good bye");
    stdout.execute(ResetColor)?;

    Ok(())
}
