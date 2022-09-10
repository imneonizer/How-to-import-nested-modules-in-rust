mod scripts;
mod utils;
mod shared;
mod foo;

fn main() {
    println!("main::main()");
    scripts::init();
    scripts::func::init();
    utils::hello::init();
    utils::world::init();

    utils::init();
    utils::hello::world();
    utils::world::hello();
    utils::hello::func();
    utils::hello::world_module_callback();

    shared::utils_hello();

    shared::init();
    shared::common::init();
    shared::common::func::init();
    shared::common::func::world();
    shared::common::func::scripts_init_callback();
    shared::common::func::foo();

    foo::init();
}