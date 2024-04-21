use chrono::{Utc, Timelike, Datelike};


fn print_datetime() {
    let now = Utc::now();
    println!(
        "Datetime now: {:02}/{:02}/{:0004} {:02}:{:02}:{:02}",
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
    let threshold: i64 = 10000000000;
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
