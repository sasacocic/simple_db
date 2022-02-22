use core::panic;
use std::io::{self, Write};
use std::process::exit;
use std::result::Result;

enum MetaCommandSuccess {
    Success,
}

// for now only have failure, but would be better to have certain ones for certain errors
enum MetaCommandFailures {
    Failure(String),
}
// this is the prepare statement??
fn execute(statement: &str) -> Result<MetaCommandSuccess, MetaCommandFailures> {
    match statement {
        statement if statement.to_lowercase().starts_with("select") => {
            Ok(MetaCommandSuccess::Success)
        }
        statement if statement.to_lowercase().starts_with("insert") => {
            Ok(MetaCommandSuccess::Success)
        }
        _ => Err(MetaCommandFailures::Failure(
            "sql wasn't valid...".to_string(),
        )),
    }
}

fn main() {
    // let mut v = Vec::new();

    // better way to write an in
    loop {
        print!("db > ");
        if let Err(err) = std::io::stdout().flush() {
            panic!("error flushing becuase: {:?}", err);
        }

        // there's a difference between String and &str don't forget that
        let mut buf = String::new();

        if let Err(error) = io::stdin().read_line(&mut buf) {
            panic!("there's an error {:?}", error);
        }

        match buf.trim() {
            buf if buf.starts_with(".") => match buf {
                ".exit" => exit(0),
                _ => println!("woah yeah no idea what you entered. Try using .help"),
            },
            buf => {
                match execute(buf) {
                    Ok(MetaCommandSuccess::Success) => {
                        // would need to return the result here
                    }
                    Err(MetaCommandFailures::Failure(message)) => {
                        println!("failure: {:?}", message);
                    }
                }
                // it's not a meta command so we should be able to do something with it here
                println!("do somethign cause this won't do anything");
            }
        }
    }
}
