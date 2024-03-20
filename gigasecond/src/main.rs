use gigasecond::after;
use time::{OffsetDateTime, PrimitiveDateTime};

fn main() {
    let today = OffsetDateTime::now_utc();
    let today = PrimitiveDateTime::new(today.date(), today.time());

    let gigasecond_later = after(today);

    println!("{gigasecond_later}")
}
