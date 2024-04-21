use chrono::{Datelike, Timelike, Utc};

fn print_datetime() {
    let now = Utc::now();
    println!(
        "Datetime now: {}/{}/{} {}:{}:{}",
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    );
}

fn main() {
    print_datetime();
    let mut count: i64 = 0;
    let threshold: i64 = 9999999999;
    loop {
        count = count + 1;
        if count % threshold == 0 {
            println!("{}", threshold);
            print_datetime();
            break;
        }
        // println!("count: {}", count);
    }
}
