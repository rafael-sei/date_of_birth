use chrono::prelude::*;

fn main() {
    let ln = Local::now().naive_local();
    let by: i64 = 2011;
    let bm: i64 = 8;
    let bd: i64 = 13;
    let age: i64;
    let h: i64 = 1988;
    // let s: i64 = 1925;

    if bm < ln.month() as i64 {
        age = ln.year() as i64 - by;
    } else if bm == ln.month() as i64 {
        if bd <= ln.day() as i64 {
            age = ln.year() as i64 - by;
        } else {
            age = ln.year() as i64 - by - 1;
        }
    } else {
        age = ln.year() as i64 - by - 1;
    }

    let h = by - h;

    println!("西暦{}年、和暦{}年。年齢は{}歳。", by,h,age);
}
