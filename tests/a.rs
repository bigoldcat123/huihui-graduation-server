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

#[tokio::test]
async fn feature2() {
    let client = reqwest::Client::new();
    let part = reqwest::multipart::Part::bytes(vec![1,2,3,4,5,])
        .file_name("a.png");

    let form = reqwest::multipart::Form::new()
        .part("image", part);

    let res = client
        .post("http://127.0.0.1:8080/image")
        .multipart(form)
        .send()
        .await.unwrap();
    let res = res.text().await.unwrap();
    println!("res -> {res}");
}
