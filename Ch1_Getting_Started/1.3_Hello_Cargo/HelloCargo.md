# All the Cargo
Cargo is Rusts build system and package manager. cargo will build projects, download libraries and build dependencies.

### Creating a project with Cargo

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

This will create a local directory with VCS already set up (git). We can open the project Cargo.toml to update things about the project. This command will create a cargo project with a `.gitignore`, `Cargo.toml` and a `src` directory all inside a directory that matches the name of the cargo project

### Building and running a Cargo Project
```bash
cargo build
```
 Will build and compile the cargo project.


 ```bash
 cargo run
 ```
 Will run the cargo project

 You can use `cargo check` to check that the program compiles without creating binary executables.

 `cargo build --release` This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.
