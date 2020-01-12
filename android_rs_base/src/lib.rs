mod app_container;
mod app_implementor;
mod app_config;
mod input;
mod storage;

pub use self::app_config::*;
pub use self::app_container::*;
pub use self::app_implementor::*;
pub use self::storage::*;
pub use self::input::InputEvent;

// Useful to have pre-imported

pub use piston::input::{RenderArgs, UpdateArgs};

/// Sets RUST_BACKTRACE=1 to enable backtraces in android, useful to get backtraces
pub fn enable_backtrace() {
    use std::env;
    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
}
