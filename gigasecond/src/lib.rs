use time::{Duration, PrimitiveDateTime as DateTime};

pub const KILO: u16 = 1000;
pub const MEGA: u32 = 1000 * KILO as u32;
pub const GIGA: u32 = 1000 * MEGA as u32;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start.checked_add(Duration::seconds(GIGA as i64)).unwrap();
}
