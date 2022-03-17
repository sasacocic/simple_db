use arr_macro::arr;
use core::panic;
use std::io::{self, Write};
use std::process::exit;
use std::result::Result;
mod table;
use table::{testing, Page, Table, MAX_TABLE_ROWS};

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
    if table.num_rows > MAX_TABLE_ROWS {
        return Err(MetaCommandFailures::Failure(
            "max table rows has been reached".to_string(),
        ));
    }

    match statement {
        statement if statement.to_lowercase().starts_with("select") => {
            // println!("num rows: {}", table.num_rows);
            for page in table.pages.iter() {
                if page.rows.is_some() {
                    let rows = page.rows.as_ref().expect("there to be a row");

                    for row in rows.iter() {
                        if row.is_none() {
                            break;
                        }

                        let row = row.as_ref().unwrap();
                        println!("{:?}", row);
                    }
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
            // println!("table {:?}", table);
            Ok(MetaCommandSuccess::Success)
        }
        _ => Err(MetaCommandFailures::Failure(
            "sql wasn't valid...".to_string(),
        )),
    }
}

fn main() {
    testing("testing".to_string());
    let mut table = Table {
        num_rows: 0,
        pages: arr![Page { rows: None }; 100], // 100 === MAX_TABLE_PAGES
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
                        println!("failure: {}", message);
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
        /*
        - basically main problem right now is that the way stdout "looks" isn't nesecarily the way
        it's displayed
        */

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
            r#"db > (id:1,username:hello,email:hello@gmail.com)"#,
            r#"(id:2,username:sasa,email:sasa@gmail.com)"#,
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
                    .expect("expected stdout")
                    .read_to_string(&mut stdout)
                    .expect("thing is read to stdout");

                // print!("stdout of process: {:?}", stdout);

                // assert_eq!(stdout, result);
                let formatted = format!("{}", stdout);
                println!("left {}", format!("{}", stdout));
                println!("right {}", format!("{}", &result));
                println!("len left {}, len right {}", formatted.len(), result.len());
                assert_eq!(format!("{}", stdout), result);
                child.wait_with_output().expect("this is finish just fine");

                return Ok(());
            }
            Err(err) => {
                eprintln!("{}", err);
                panic!("not good it paniced {}", err);
            }
        }
    }

    // #[test]
    // fn insert_string_longer_than_max() -> Result<(), String> {
    //     unimplemented!();
    // }
}
