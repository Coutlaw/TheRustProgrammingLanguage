# Anatomy of a Rust Program
to define a function in Rust
```rs
fn main(){
	// function
}
```
The main function is like every other main function, its the first code executed in a Rust program. Rust requires {} for scope boundaries. 

`rustfmt` is a tool that will format you code in a particular style. find out more here https://github.com/rust-lang/rustfmt

### The function anatomy

```rs
println!("hello, world!");
```
`println!` calls a rust *macro*, to call the function you could use `println` without the `!`, functions and macros are different but more to come on that later.

### Compile and Run
compile and run rust with `rustc {filename}.rs` and run the binary executable.
