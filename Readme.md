### Rust Nested Imports

**Steps to run**

````sh
chmod +x ./run.sh
bash ./run.sh
````

- Directory structure:

  ```sh
  src
  ├── foo.rs
  ├── main.rs
  ├── scripts
  │   ├── func.rs
  │   └── mod.rs
  ├── shared
  │   ├── common
  │   │   ├── func.rs
  │   │   └── mod.rs
  │   └── mod.rs
  └── utils
      ├── hello.rs
      ├── mod.rs
      └── world.rs
  ```

- The entry point for rust application is `src/main.rs` it is mandatory to have `fn main()` inside it.

- Anything that you want to import should first be defined as module

  - As in the `main.rs` there are four module imports, in other words four modules are defined:

    ```rust
    mod scripts;
    mod utils;
    mod shared;
    mod foo;
    ```

  - The three modules `scripts, utils, shared` are directories while `foo` is a file.

  - For each module which is a directory, It is mandatory to have `mod.rs` inside it.

- All functions / sub modules are accessed via `::` path separator, eg: `foo::init()`.

- Once the module is defined, It can be used inside any file using same module name, for eg: the file `src/shared/common/func.rs` is able to use `utils::hello::world` which is defined at `src/utils/hello.rs`, pay attention to `use` keyword for already defined modules.

- Module can also be imported with a different alias to avoid name conflicts, such an example is available inside `src/shared/common/func.rs`  i.e., `use scripts::init as scripts_init;`

- To use any file present in current file directory of sub module, `super` keyword can be used, this comes in handy if you don't want to hard code the module directory name. A similar pattern can be notice in python such as `from .foo import foo`. such an example is available inside `src/utils/hello.rs`

  