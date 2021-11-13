Credit- Brad Traversy

# Introduction to RUST Programming

Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

## Quick Start

Un-comment the file function to run

```bash
# Run With Cargo
cargo run

# Build
cargo build

# Build for production
cargo build --release
```

## Variables in RUST
Variables hold primitive data or references ot data
Variables are immutable by default in rust
Rust is a block-scoped language

```bash
pub fn run() {
    let name = "Vijendr";
    let mut age = 37; //making age mutable using keyword mut
    age = 39;
    println!("My Name is {} and I am {}", name, age);
    //Define constant
    const ID: i32 = 001;
    println!("ID:{}", ID);

    //Asssign multiple vars
    let (my_name, my_age) = ("Vijendr", 37);
    println!("{} is {}", my_name, my_age);
}
```

## Data Types:
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays

```bash
pub fn run() {
    // Default is "i32"
    let x = 1;
    // Default is "f64"
    let y = 2.5;
    // Add explicit type
    let z: i64 = 4545445454545;
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    // Boolean
    let is_active: bool = true;
    // Get boolean from expression
    let is_greater: bool = 10 < 5;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
```

## Reference Types
Reference Pointers - Point to a resource in memory
With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You will need to use a reference (&) to point to the resource
```bash

   //Vector Type
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

```

## Tuples

Tuples group together values of different types
Max 12 elements

```bash

let person: (&str, &str, i8) = ("Vijendr", "Mars", 37);
println!("{} is from {} and is {}", person.0, person.1, person.2);

```

## Struct

Structs - Used to create custom data types

```bash
Traditional Struct
 struct Color {
   red: u8,
   green: u8,
   blue: u8,
 }

 Tuple Struct
 struct Color(u8, u8, u8);
```

## Vectors

```bash
// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}

```

## Enums

Enums are types which have a few definite values

```bash
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
  }

  fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
      Movement::Up => println!("Avatar moving up"),
      Movement::Down => println!("Avatar moving down"),
      Movement::Left => println!("Avatar moving left"),
      Movement::Right => println!("Avatar moving right"),
    }
  }

  pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
  }

```

## Strings

Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or own string data

```bash
pub fn run() {
    let mut hello = String::from("Hello ");
    // Get length
    println!("Length: {}", hello.len());
    // Push char
    hello.push('W');
    // Push string
    hello.push_str("orld!");
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    // Check if empty
    println!("Is Empty: {}", hello.is_empty());
    // Contains
    println!("Contains 'World' {}", hello.contains("World"));
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));
    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}

```

## Arrays:

Arrays - Fixed list where elements are the same data types

```bash
use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}

```

## Functions:
Functions - Used to store blocks of code for re-use

``` bash
pub fn run() {
    greeting("Hello", "Jane");
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);
    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 { // Returning Integer Value 
    n1 + n2
}

```
