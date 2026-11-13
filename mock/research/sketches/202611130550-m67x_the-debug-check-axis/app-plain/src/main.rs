//! The plain consumer alone in the build.

use std::panic::{catch_unwind, set_hook};

fn report(arm: &str, f: fn() -> u8) {
    match catch_unwind(f) {
        Ok(v) => println!("plain-user, {arm}: completes to {v}"),
        Err(_) => println!("plain-user, {arm}: aborts"),
    }
}

fn main() {
    set_hook(Box::new(|_| {}));
    report("by feature", plain_user::by_feature);
    report("by parameter", plain_user::by_parameter);
}
