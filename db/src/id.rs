use rand::prelude::*;

pub fn new_id() -> String {
    let value: u64 = random();
    let hex_value = format!("{:x}", value);
    hex_value
}
