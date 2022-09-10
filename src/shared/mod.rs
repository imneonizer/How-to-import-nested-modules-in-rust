pub mod common;
use utils::hello;

pub fn init() {
    println!("shared::init()")
}

pub fn utils_hello() {
    hello::init();
}