
extern crate time;

use time::now;

fn main() {

    let time = now();
    println!(
        "{month}.{day}.{year}",
        month=(time.tm_mon + 1),
        day=time.tm_mday,
        year=(time.tm_year + 1900)
    );
}
