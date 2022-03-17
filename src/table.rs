use core::panic;
use std::mem::{size_of, size_of_val};

// - store rows in blocks of memory called pages
// page size should be 4096? 4mb * 100 => 400mb
// max pages = 100
pub const MAX_TABLE_ROWS: u16 = 4096;
pub const MAX_TABLE_PAGES: usize = 100;
pub const PAGE_SIZE: usize = 4096;
pub const ROW_SIZE: usize = std::mem::size_of::<Row>();
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

#[derive(Debug, Clone)]
pub struct Row {
    pub id: i64,
    pub username: [u8; 32],
    pub email: [u8; 255],
}

// 291 * 14 = 4,074
/*
a bit werid because the struct isn't actually 4,074 becuase
strings in rows can grow larger than expected sooo.....
*/
#[derive(Debug, Clone)]
pub struct Page {
    pub rows: Option<[Option<Row>; 14]>,
}
/*
Table
    Pages - size 4096 Kb - 100 Pages
    {
        [Page; 4096]
    }
        Page
        {
            [Row; 291] // because 291 * 14 -> 4,074
        }
            Row - size 291 bytes - as many as can fit into a page
            {
                id, (integer)
                username, (varchar 32)
                email (varchar 255)
            }


*/

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
            username: actual_username,
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
pub fn testing(sql_stmnt: String) -> i32 {
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
