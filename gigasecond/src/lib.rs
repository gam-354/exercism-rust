use time::PrimitiveDateTime as DateTime;
use time::Duration;
use std::ops::Add;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::new(1_000_000_000,0))
}
