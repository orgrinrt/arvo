//! The plain consumer beside one that turned the feature on.

use std::panic::{catch_unwind, set_hook};

fn report(who: &str, arm: &str, f: fn() -> u8) {
    match catch_unwind(f) {
        Ok(v) => println!("{who}, {arm}: completes to {v}"),
        Err(_) => println!("{who}, {arm}: aborts"),
    }
}

fn main() {
    set_hook(Box::new(|_| {}));
    report("plain-user", "by feature", plain_user::by_feature);
    report("plain-user", "by parameter", plain_user::by_parameter);
    report("quiet-user", "by parameter", quiet_user::by_parameter);
}
