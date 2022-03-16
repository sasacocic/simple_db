use core::panic;
use std::io::{self, Write};
use std::process::exit;
use std::result::Result;
mod table; // tells rust to load this module
use table::{testing, Page, Pages, Row, Table, MAX_TABLE_PAGES, MAX_TABLE_ROWS};

/*
putting limitations on our db
- support two operations: insert a row and printing all rows
- reside only in memory (no disk persistance)
- support a single, hard coded table
    which will look like
    - id - integer
    - username - varchar(32)
    - email - varchar(255)


example insert statement: insert 1 cstack foo@bar.com OR insert 1 sasacocic sasacocic@gmail.com
*/

/*
TODOS
- store rows in blocks of memory called pages
- each page stores as many rows as it can fit
- rows are serialized into a compact representation with each page
- pages are only allocated as needed
- keep a fixed-size array of pointers to pages
*/

/*
- basically here's what I need to do
- just store this shit in memory
*/

// maybe make an 'trait' for table??
// Obviously username and email should be arrays or vecs or something else to reflect their actual size

enum MetaCommandSuccess {
    Success,
}

// size of a row should be ~= 291 bytes

// for now only have failure, but would be better to have certain ones for certain errors
enum MetaCommandFailures {
    Failure(String),
}

// this is the prepare statement??
fn execute(statement: &str, table: &mut Table) -> Result<MetaCommandSuccess, MetaCommandFailures> {
    if table.numRows > MAX_TABLE_ROWS {
        return Err(MetaCommandFailures::Failure(
            "max table rows has been reached".to_string(),
        ));
    }

    match statement {
        statement if statement.to_lowercase().starts_with("select") => {
            for page in table.pages.0.iter() {
                if page.0.is_some() {
                    let rows = page.0.as_ref().unwrap();
                    let mut count = 0;

                    // while let Some(Some(row)) = rows.iter().next() {
                    for row in rows.iter() {
                        if row.is_none() {
                            break;
                        }
                        let row = row.as_ref().unwrap();
                        println!(
                            "{}: id: {:?}, username: {:?}, email: {:?}",
                            count, row.id, row.username, row.email
                        );
                        count += 1;
                    }
                    // }
                } else {
                    break;
                }
            }
            Ok(MetaCommandSuccess::Success)
        }
        statement if statement.to_lowercase().starts_with("insert") => {
            // ok literally only worry about the current case so spliting things should be fine
            let statement = statement.to_string();
            let statement_split = statement.split(" ");
            // println!("{:?}", statement_split);

            //let stringVer: Row = statement_split.into();
            //print!("frommed row: {:?}", stringVer);

            // serialize row
            let coled: Vec<&str> = statement_split.collect();

            table.insert_row(coled).unwrap();
            table.numRows += 1;
            Ok(MetaCommandSuccess::Success)
        }
        _ => Err(MetaCommandFailures::Failure(
            "sql wasn't valid...".to_string(),
        )),
    }
}

fn main() {
    testing("insert 1 sasa sasa@coci.com".to_string());

    exit(99);

    let mut table = Table {
        numRows: 0,
        pages: Pages(vec![Page(None); MAX_TABLE_PAGES]),
    };

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
                match execute(buf, &mut table) {
                    Ok(MetaCommandSuccess::Success) => {
                        // would need to return the result here
                        print!("{}", "success\n");
                    }
                    Err(MetaCommandFailures::Failure(message)) => {
                        println!("failure: {:?}", message);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        process::Command,
        process::Stdio,
    };

    #[test]
    fn check_input() -> Result<(), String> {
        let inputs = [
            "insert 1 hello hello@gmail.com\n",
            "insert 2 sasa sasa@gmail.com\n",
            "select stuff\n",
            ".exit\n",
        ];

        let res = Command::new("./target/debug/simple_db")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        let result = [
            "db > success",
            "db > success",
            r#"db > 0: id: 1, username: "hello", email: "hello@gmail.com""#,
            r#"1: id: 2, username: "sasa", email: "sasa@gmail.com""#,
            "success",
            "db > ",
        ]
        .join("\n");

        match res {
            Ok(mut child) => {
                let stdin = child.stdin.as_mut().expect("to get mutable stdin"); // crash of none

                for input in inputs.iter() {
                    stdin
                        .write(input.as_bytes())
                        .expect(format!("{} to write", input).as_str()); // crash if no
                }

                stdin.flush().expect("flush correct");

                let mut stdout = String::new();
                child
                    .stdout
                    .as_mut()
                    .expect("stdout")
                    .read_to_string(&mut stdout)
                    .expect("thing is read to stdout");

                // print!("stdout of process: {:?}", stdout);

                assert_eq!(stdout, result);
                child.wait_with_output().expect("this is finish just fine");

                return Ok(());
            }
            Err(err) => {
                panic!("not good it paniced");
            }
        }
    }

    #[test]
    fn insert_string_longer_than_max() -> Result<(), String> {
        unimplemented!();
    }
}
