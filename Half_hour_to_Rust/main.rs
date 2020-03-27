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








