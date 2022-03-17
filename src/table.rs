use core::panic;
use std::fmt::Debug;
use std::fmt::{self};
use std::str::from_utf8;

pub const MAX_TABLE_ROWS: u16 = 4096;

#[derive(Clone)]
pub struct Row {
    pub id: i64,
    pub username_length: usize,
    pub email_length: usize,
    pub username: [u8; 32],
    pub email: [u8; 255],
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let username = &from_utf8(&self.username).unwrap_or("[username]")[0..self.username_length];
        let email = &from_utf8(&self.email).unwrap_or("[email]")[0..self.email_length];
        let id = self.id.clone();
        let f_str = format!("(id:{},username:{},email:{})", id, username, email);
        f.write_str(f_str.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub rows: Option<[Option<Row>; 14]>,
}

#[derive(Debug)]
pub struct Table {
    pub num_rows: u16,
    pub pages: [Page; 100],
}

impl<'a> Table {
    // input arg should be a row, but using a vec for now
    pub fn insert_row(&mut self, row: Vec<&'a str>) -> Result<i32, &'a str> {
        let id: i64 = row.get(1).unwrap().parse().unwrap();
        let username = row.get(2).unwrap().as_bytes();
        let email = row.get(3).unwrap().as_bytes();

        let mut actual_username: [u8; 32] = [0; 32];
        let mut actual_email: [u8; 255] = [0; 255];

        for (ind, val) in username.into_iter().enumerate() {
            actual_username[ind] = *val;
        }

        for (ind, val) in email.into_iter().enumerate() {
            actual_email[ind] = *val;
        }

        let r = Row {
            id: id,
            username_length: username.len(),
            username: actual_username,
            email_length: email.len(),
            email: actual_email,
        };

        // 14 is the number of rows that go into a page
        let page_index = self.num_rows as usize / 14;
        let row_index = self.num_rows as usize % 14;

        let page = &mut self.pages[page_index];

        // println!(
        //     "inserting row {:?} at page: {} & row: {}",
        //     &r, page_index, row_index
        // );

        // println!("page {:?}", page);

        match page.rows.as_mut() {
            Some(rows) => {
                if rows[row_index].is_none() {
                    rows[row_index] = Some(r);
                    // println!("page after insert {:?}", rows);
                    // println!("table page after insert {:?}", self.pages[page_index]);
                } else {
                    println!(
                        "row {:?} && row.get(row_index) {:?}",
                        rows,
                        rows.get(row_index)
                    );
                    panic!("not adding somehow ..")
                }
                // arr_of_rows[row_index] = Some(r);
                self.num_rows += 1;
            }
            None => {
                // println!("None branch running");
                // need to create the page
                let mut new_page: Page = Page {
                    rows: Some([
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None,
                    ]),
                };
                new_page.rows.as_mut().unwrap()[0] = Some(r);
                // println!("new page being inserted {:?}", new_page);
                self.pages[page_index] = new_page;
                self.num_rows += 1;
            }
        }
        return Ok(1);
    }
}

/* just messing around below here also  */
pub fn testing(_sql_stmnt: String) -> i32 {
    let mut testing: [String; 4] = [
        "helo".to_string(),
        "helo".to_string(),
        "helo".to_string(),
        "helo".to_string(),
    ];

    testing[0] = "slkdjf".to_string();

    // println!("{:?}", testing);

    return 99;
}
