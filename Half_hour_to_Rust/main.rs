// Lets Get Rusty 🦀

// ✅ you are a pro now
// 👇 is a macro
println!("Hello, world!");

let x; // declare a variable
x = 42; // x is now 42
// can be converted to 
let new_x = 42;
// same but with type annotation
let new_new_x: i32 = 42 // i32 is signed 32 bit integer

// if you declare a variable the compiler wont let you use it until it has a value

// similar to go, the _ is a throw away character
let _ = 42; // is a NOP

let _ = my_func(); // will call my_func() but throw the result away

let _x; // compiler wont yell about this value being unused because of the leading _

let y = 13;
let y = y + 1; // This is shadowing, it throws the first y away

// Rust has tuples (yay)
let pair = ('poo', 13) // annotate with let pair: (string, i32) = ('poo', 13)
pair.0 // is 'poo'
pair.1 // 13

// tuple destructing
let (some_char, some_int) = ('a', 17);
// some_char = 'a', some_int = 17

// using for return structures
let (left, right) = slice.split_at(middle);

// or you can throw bits away
let (_, part_i_care_about) = slice.split_at(middle);

// statments can be multi line
let x = vec![1,3,4,5,6,7,8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x,y| x+y);

// fn declares a function
fn greet(){
    println!("ohai");
}

// arrows indicate return types
fn fair_dice_roll() -> i32 {
    4
}

// code brackets evaluate to expressions (Concept from SICP)
let x = 42;
// is the same as
let x = { 42 };

// multi line expressions with brackets
let x = {
    let y = 1; // first statement
    let z = 2; // second statement
    y + z // this is the *tail* - what the whole block will evaluate to
};

// tail evaluation like scala
fn fair_dice_roll1() -> i32 {
    returns 4;
}
// is equal too
fn fair_dice_roll2() -> i32 {
    4
}

// pattern matching like scala 👍 very nice
fn fair_dice_roll3() -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

// dot access
let a = (10, 12);
a.0 // is 10

// method calls from a class on value
let nick = "fasterthanlime";
nice.len();


//Double colon is a namespace opperator
let least = std::cmp::min(3, 5); // std lib, compare, min function
// 👆similar to a static method in other languages

// use to affect scope
use std::cmp::min

let least = min(7,1); // this is 1

// different ways to use
// this works:
use std::cmp::min;
use std::cmp::max;

// this also works:
use std::cmp::{min, max};

// this also works!
use std::{cmp::min, cmp::max};

// wildcard will import the whole namespace
use std::cmp::*;

// Types can be namespaces too
let x = "amos".len(); // this is 4
let x = str::len("amos"); // this is also 4
// same same but different but same
// NOTE: str is a primitive always in scope

// Vec is a struct always in scope as well, so not just primitive types
let v = Vec::new();

// same as above, with full import path for visibility
let v = std::vec::Vec::new();

// rust uses the prelude at the beginning of every module
use std::prelude::v1::*; // hidden at the top of every rust program


// Declare structs
struct Vec2 {
	x: f64, // floating point 64 bit
	y: f64,
}

let v1 = Vec2 { x: 1.0, y: 3.0 };
let v2 = Vec2 { y: 2.0, x: 1.0 };
// order does not matter, names do

// struct update syntax, can only happen in last position
let v3 = Vec2 {
    x: 14.0,
    ..v2
};

// can use like this
let v4 = Vec2 { ..v3 };

// can be used for destructuring too
let Vec2 { x, .. } = v;
// this throws away `v.y`

// Using let patterns in conditions
struct Number {
    odd: bool,
    value: i32,
}

fn main() {
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    print_number(one);
    print_number(two);
}

fn print_number(n: Number) {
    // If I can make a number from n that has this prop, its true
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

// this prints:
// Odd number: 1
// Even number: 2

// Pattern matching but with structs like a let if
fn print_number(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

// this prints the same as before

// Match statments
// Matches have to be exhaustive (like scala)
fn print_number(n: Number){
    match n {
        Number { value: 1, .. } => println!("One"),
        Number { value: 2, .. } => println!("Two"),
        Number { value, .. } => println!("{}", value),
        // if that last arm didn't exist, we would get a compile-time error
    }
}

// Same as above but with wildcard catch all
fn print_number(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}

// struct methods (using impl)
impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}
// usage
let minus_two = Number {
    odd: false,
    value: -2,
};
println!("positive? {}", minus_two.is_strictly_positive());
// this prints "positive? false"

// instance struct props are immutable by default
let n = Number {
    odd: true,
    value: 17,
};
n.odd = false; // error: cannot assign to `n.odd`,
               // as `n` is not declared to be mutable
n = Number {
    odd: false,
    value: 22,
}; // error: cannot assign twice to immutable variable `n`

// make it mutable
let mut n_mutable = Number {
    odd: true,
    value: 17,
}
n_mutable.value = 19; // all good


// Traits are something shared by multiple types
trait Signed {
    fn is_strictly_negative(self) -> bool;
}
// Traits continued:
// You can implement:

//     one of your traits on anyone's type
//     anyone's trait on one of your types
//     but not a foreign trait on a foreign type

// These are called the “orphan rules”.

// Trait IMPL on Number
impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

// usage
let n = Number { odd: false, value: -44 };
println!("{}", n.is_strictly_negative()); // prints "true"

// Our trait on a primitive (foreign type)
impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

// usage
let n: i32 = -44;
    println!("{}", n.is_strictly_negative()); // prints "true"

// the `Neg` trait is used to overload `-`, the
// unary minus operator.
impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        Number {
            value: -self.value,
            odd: self.odd,
        }        
    }
}

// usage
let n = Number { odd: true, value: 987 };
let m = -n; // this is only possible because we implemented `Neg`
println!("{}", m.value); // prints "-987"


// inside an impl block, self always means that type
impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }        
    }
}
// same as one above but using Self

// Marker Traits
// some traits are markers: 
// they don't say that a type implements some methods, 
// they say that certain things can be done with a type.

// For example: i32 implements trait Copy (in short, i32 is Copy), so this works:

let a: i32 = 15;
let b = a; // `a` is copied
let c = a; // `a` is copied again
// also works because i32 is a copy
fn print_i32(x: i32) {
    println!("x = {}", x);
}

let a: i32 = 15;
print_i32(a); // `a` is copied
print_i32(a); // `a` is copied again

// 👇 this would not work
let n = Number { odd: true, value: 51 };
let m = n; // `n` is moved into `m`
let o = n; // error: use of moved value: `n`
// seems to be a pass by value, but terminates the original value

// what if we tried a function?
fn print_number(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}
let n = Number { odd: true, value: 51 };
print_number(n); // `n` is moved
print_number(n); // error: use of moved value: `n`
// same error as before

// lets pass a reference to borrow n
fn print_number2(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

let n = Number { odd: true, value: 51 };
print_number2(&n); // `n` is borrowed for the time of the call
print_number2(&n); // `n` is borrowed again
// works with immutable reference

// applying MUTABILITY
fn invert(n: &mut Number) {
    n.value = -n.value;
}

fn print_number3(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

// this time, `n` is mutable
let mut n = Number { odd: true, value: 51 };
print_number(&n);
invert(&mut n); // `n is borrowed mutably - everything is explicit
print_number(&n); // we have changed n with the invert function

// we can reference self for the same behavior (add mut for mutability)
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

let n = Number { odd: true, value: 51 };
let mut m = n.clone();
m.value += 100;

print_number(&n);
print_number(&m);

// since receivers are borrowed implicitly these two are equal
let m = n.clone();

let m = std::clone::Clone::clone(&n);

// note: `Copy` requires that `Clone` is implemented too
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
// Marker traits have no methods
impl std::marker::Copy for Number {}

// now with the marker trait we can do this
let n = Number { odd: true, value: 51 };
let m = n; // `m` is a copy of `n`
let o = n; // same. `n` is neither moved nor borrowed.

// Common traits can be implemented with the derive attribute
#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

// this expands to `impl Clone for Number` and `impl Copy for Number` blocks.

// Generic Functions
fn foobar<T>(arg: T) {
    // do something with `arg`
}

// They can have multiple type parameters, which can then be used in the function's declaration and its body, instead of concrete types:
fn foobar<L, R>(left: L, right: R) {
    // do something with `left` and `right`
}

// Type params usually have constraints
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}
// These 👆:are equivalent to 👇
fn print<T>(value: T)
where
    T: Display,
{
    println!("value = {}", value);
}
// 👆long form constraints

// more complex constraints
fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

// usage 
compare("tea", "coffee");
// prints: "tea" != "coffee"

// Structs can be generic
struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

let p1 = Pair { a: 3, b: 9 };
let p2 = Pair { a: true, b: false };
print_type_name(&p1); // prints "Pair<i32>"
print_type_name(&p2); // prints "Pair<bool>"

// std lib has a Vec wich is generic
let mut v1 = Vec::new();
v1.push(1);
let mut v2 = Vec::new();
v2.push(false);
print_type_name(&v1); // prints "Vec<i32>"
print_type_name(&v2); // prints "Vec<bool>"

// more usage of vector
let v1 = vec![1, 2, 3];
let v2 = vec![true, false, true];
print_type_name(&v1); // prints "Vec<i32>"
print_type_name(&v2); // prints "Vec<bool>"

// Macros just expand to regular code
println!("Hello there!");
// expands too
use std::io::{self, Write};
io::stdout().lock().write_all(b"Hello there!\n").unwrap();

// Panic is also a macro
panic!("This panics");

// OPTIONS (basically the same as scala)
let o1: Option<i32> = Some(128);
o1.unwrap(); // this is fine

let o2: Option<i32> = None;
o2.unwrap(); // this panics!

// option is an enum not a struct
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

use self::Option::{None, Some};

// this is like result which is also an enum 
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// LIFETIMES
// `x` doesn't exist yet
{
    let x = 42; // `x` starts existing
    println!("x = {}", x);
    // `x` stops existing
}
// `x` no longer exists

// references also have a lifetime
// `x` doesn't exist yet
{
    let x = 42; // `x` starts existing
    let x_ref = &x; // `x_ref` starts existing - it borrows `x`
    println!("x_ref = {}", x_ref);
    // `x_ref` stops existing
    // `x` stops existing
}
// `x` no longer exists

// lifetime of a reference cannot exceed the lifetime of the variable binding it borrows
let x_ref = {
    let x = 42;
    &x
};
println!("x_ref = {}", x_ref);
// error: `x` does not live long enough

// a variable binding can be immutably borrowed multiple times
let x = 42;
let x_ref1 = &x;
let x_ref2 = &x;
let x_ref3 = &x;
println!("{} {} {}", x_ref1, x_ref2, x_ref3);

// however a mutatable variable cannot be mutated while borrowed
let mut x = 42;
let x_ref = &x;
x = 13;
println!("x_ref = {}", x_ref);
// error: cannot assign to `x` because it is borrowed

//While immutably borrowed, a variable cannot be mutably borrowed:
let mut x = 42;
let x_ref1 = &x;
let x_ref2 = &mut x;
// error: cannot borrow `x` as mutable because it is also borrowed as immutable
println!("x_ref1 = {}", x_ref1);

// function references 
fn print(x: &i32) {
    // `x` is borrowed (from the outside) for the
    // entire time this function is called.
}


