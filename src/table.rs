use std::marker::Copy;
use std::mem::{size_of, size_of_val};
use std::str::Split;

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
    pub username: String,
    pub email: String,
}

// I split on spaces so will get something that
// looks like ["insert", "1", "sasa", "sasacocic"]
impl<'a> From<Split<'a, &str>> for Row {
    fn from(stmnt: Split<&str>) -> Self {
        let veced: Vec<&str> = stmnt.collect();
        return Row {
            id: 1,
            username: "hello".to_string(),
            email: "hello".to_string(),
        };
    }
}

// 291 * 14 = 4,074
/*
a bit werid because the struct isn't actually 4,074 becuase
strings in rows can grow larger than expected sooo.....
*/
#[derive(Debug, Clone)]
pub struct Page(pub Option<[Option<Row>; 14]>);

#[derive(Debug, Clone)]
pub struct Pages(pub Vec<Page>);

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

pub struct Table {
    pub numRows: u16,
    pub pages: Pages,
}
// pub struct Table<T> {
//     pub numRows: u16,
//     pub pages: [T; MAX_TABLE_PAGES],
// }
//
// impl<'a> Table<Option<Row>> {
//     // input arg should be a row, but using a vec for now
//     pub fn insert_row(&mut self, row: Vec<&'a str>) -> Result<i32, &'a str> {
//         let id: i64 = row.get(1).unwrap().parse().unwrap();
//         let username = row.get(2).unwrap().to_string();
//         let email = row.get(3).unwrap().to_string();
//         let r = Row {
//             id: id,
//             username,
//             email,
//         };
//         // which page does this thing need to be added to?
//         // well I should just be adding rows to each page until the page is full
//         // I want to insert this row in the right page....
//         // self.pages.push(r);
//         return Ok(1);
//     }
// }

impl<'a> Table {
    // input arg should be a row, but using a vec for now
    pub fn insert_row(&mut self, row: Vec<&'a str>) -> Result<i32, &'a str> {
        let id: i64 = row.get(1).unwrap().parse().unwrap();
        let username = row.get(2).unwrap().to_string();
        let email = row.get(3).unwrap().to_string();
        let r = Row {
            id: id,
            username,
            email,
        };

        // insert in the first free place. Since it's an
        // array this is be the first place where it isn't None
        let mut page_count = 0;
        // the loop should always break.... if it doesn't then I fucked up
        loop {
            let page = &mut self.pages.0[page_count].0;
            match page {
                Some(things) => {
                    if (things.iter().any(|e| e.is_none())) {
                        break;
                    } else {
                        page_count += 1
                    };
                }
                None => break,
            }
        }
        match &mut self.pages.0[page_count] {
            Page(Some(arr_of_row)) => {
                // unwrapping here should always be ok, because I found the first thing??
                let pos = arr_of_row.iter().position(|e| e.is_none()).unwrap();
                arr_of_row[pos] = Some(r);
            }
            Page(None) => {
                self.pages.0[page_count] = Page(Some([
                    Some(r),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]))
            }
        }
        return Ok(1);
    }
}

// pub fn row_slot(table: &mut NewTable<Row>, row_num: usize) {
//     let page_numb = row_num / ROWS_PER_PAGE;
//     match table.pages.get(page_numb) {
//         Some(row) => {}
//         None => {
//             // need to allocate a new row....
//             // with the array I
//         }
//     }
// }

pub fn testing(sql_stmnt: String) -> i32 {
    return 99;
}
