// src/utils.rs

use log::{error};
pub fn log_error(message: &str) {
    let location = std::panic::Location::caller();
    error!("Error at {}:{}: {}",location.file(),location.line(), message);
}