use utils;
use scripts;
use super::world as world_module; // import from current directory

pub fn init(){
    println!("utils::hello::init()");
}

pub fn h(){
    utils::world::h();
}

pub fn world(){
    utils::world::w();
}

pub fn func(){
    scripts::func::f();
}

pub fn world_module_callback(){
    world_module::w();
}