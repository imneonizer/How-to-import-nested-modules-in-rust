use utils;
use foo;
use scripts::init as scripts_init;

pub fn init(){
    println!("shared::common::func::init()")
}

pub fn world(){
    utils::hello::world();
}

pub fn foo(){
    foo::init();
}

pub fn scripts_init_callback(){
    scripts_init();
}