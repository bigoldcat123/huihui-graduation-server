use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, Offset};

#[test]
fn feature() {
    let occurred_at = NaiveDate::parse_from_str("2022-10-20", "%Y-%m-%d")
        .unwrap();
    let occurred_at = occurred_at.and_hms(0, 0, 0);
    let occurred_at:DateTime<Local> = DateTime::from_naive_utc_and_offset(occurred_at, *Local::now().offset());
    println!("{occurred_at:?}");
    let t = Local::now();
    println!("{}",t);
}
