
# Cheat Sheet

## Basic Syntax

Assign variables:
```rust
let a: u32 = 42; // immutable
let mut b: u32 = 42; // mutable
```

If-Else:
```rust
let a = if 1 < 2 { // optionally use as expression
    true // last statement without ; is the result of this branch
} else if 3 < 2 { // else if does also exist
    false
} else {
    true
}; // as this is used as an expression, ; is mandatory
```

If-let:
```rust
let a = Some(42); // package the number in a Some
if let Some(number) = a { // pattern match with any pattern
    assert_eq!(number, 42); // use the captured value(s)
}
assert_eq!(a, Some(42));
```

Match-clauses:
```rust
// like a switch in C with super powers
let b = match my_lucky_number {
    42 => ..., // have a direct expression (ends with ,)
    69 => {...} // can also have a block (does not end with ,)
    1..=10 => ..., // can use ranges
    n if n % == 0 => ..., // can use arbitrary conditions
    100 | 101 => ..., // can use | to combine patterns
    _ => ..., // match always needs to be exhaustive, this is similar to a default branch
}; // same as above, match used as expr -> needs to end in ;
```

For-loop:
```rust
for i in 0..10 { ... } // equivalent to in C `for (int i = 0; i < 10; i++) {}`
for c in "hello world".chars().take(5) { ... } // use any iterator
```

While-loop:
```rust
let mut a = 0;
while a < 10 { a += 1; } // notice no ()
// iterator is Some while there are is more to come
let mut iterator = "hello world".chars(); 
while let Some(c) = iterator.next() { println!("{c}"); } // repeat while the pattern matches
```

Infinite-loop:
```rust
loop { ... }
```

Functions:
```rust
fn add(a: i32, b: i32) -> i32 { return a + b; }
fn add2(a: i32, b: i32) -> i32 { a + b } // last expression is returned if not terminated with ;
pub fn main() { ... } // public main, entrypoint to the program
pub fn main() -> () { ... } // () is no return, equivalent to previous
pub fn main() -> Result<(), MyError> { ... } // fallible main
```

## Compound types

Tuples:
```rust
let a: (u32, String) = (999, "Santa".to_string());
println!("first: {}, second: {}", a.0, a.1);
```

Structs:
```rust
struct MyEmptyStruct; // unit struct (zero size)
struct MyTupleStruct(u32, String); // contains only a list of items
struct MyStruct1 { age: u32, name: String } // give a name to the items
pub struct MyStruct2 { age: u32, name: String } // it may be public
pub struct MyStruct3 { pub age: u32, pub name: String } // fields may be public

let a = MyStruct1 {
    age: 999,
    name: "Santa".to_string(), // trailing comma optional
};
```

Enums: combination of class enums (C++/Java) and tagged unions (C/C++)
```rust
enum MyEnum {
    Empty,
    ThisIsATuple(u32, String),
    ThisIsAStruct {
        age: u32,
        name: String,
    }
}
let a = MyEnum::ThisIsAStruct {
    age: 999,
    name: "Santa".to_string(),
};
```

Arrays:
```rust
let arr: [i32; 3] = [1, 2, 3];
println!("{:?}", arr); // Print debug representation of the array
println!("[{}, {}, {}]", arr[0], arr[1], arr[2]);
let [a, b, c] = arr; // Array destructuring
println!("[{a}, {b}, {c}]");
```

## The 3 rules of ownership

1. In Rust there is always a single owner for each stack value
1. Once the owner goes out of scope any associated values should be cleaned up (drop)
1. Copy types creates copies (implicitly), all other types are moved

```rust
fn main() {
    let first_owner = String::from("Hello world");
    let new_owner = first_owner; // first_owner goes here out of scope
    // println!("{}", first_owner); // <-- ERROR, first_owner no longer exists
    let copy_of_owner = new_owner.clone();
    println!("new_owner: {}", new_owner);
    println!("copy_of_owner: {}", copy_of_owner);
}
```

## The 4 rules of borrowing

1. One mutable reference at the same time
1. Any number of immutable references at the same time as long as there is no mutable reference
1. References cannot live longer than their owners
1. A reference will always point to a valid value

```rust
let mut s = String::from("Hello ");
{ // -> start a new scope
    let ref_to_s = &mut s;
    *ref_to_s += "world";
    // println!("{}", &s); // ERROR: RULE 2
    println!("{}", ref_to_s);
} // <- ref_to_s goes of scope
println!("{}", &s);
drop(s); // <- s goes out of scope
// println!("{}", &s); // ERROR: RULE 3
```

## Generics
```rust
use std::ops::Add; // import trait, so it can be used

// use the derive-macro to auto implement traits
#[derive(Clone, Copy, Debug, PartialEq)]
// add generic parameter with the constraint that it needs to implement Add<Rhs=T>
pub struct Point<T: Add<T>> { pub x: T, pub y: T }
// Rust requires us to constrain T for a impl block
impl<T: Add<T>> Point<T> {
    pub fn new(x1: T, y: T) -> Self {
        Self {
            x: x1,
            y, // => shorthand notation 
        } // no ; as we are returning
    }
}

impl<T> Add<Self> for Point<T>
where
    T: Add<T, Output=T> // alternative way to constrain T
{ 
    type Output = Self; // associated type, like in Scala

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

let a = Point::new(1, 2);
let b = Point::new(3, 4);
let c = a + b;
println!("x: {}, y: {}", c.x, c.y);
```

## Traits
```rust
// define new trait
pub trait Numeric {
    // define static function
    fn zero() -> Self;
}

// define trait that requires the implementation of another type
// This trait is used as a marker for floats
pub trait Floaty: Numeric { }

impl Numeric for u8 { fn zero() -> Self { 0 } }
impl Numeric for u16 { fn zero() -> Self { 0 } }
impl Numeric for u32 { fn zero() -> Self { 0 } }
impl Numeric for u64 { fn zero() -> Self { 0 } }
impl Numeric for u128 { fn zero() -> Self { 0 } }
impl Numeric for usize { fn zero() -> Self { 0 } }

impl Numeric for i8 { fn zero() -> Self { 0 } }
impl Numeric for i16 { fn zero() -> Self { 0 } }
impl Numeric for i32 { fn zero() -> Self { 0 } }
impl Numeric for i64 { fn zero() -> Self { 0 } }
impl Numeric for i128 { fn zero() -> Self { 0 } }
impl Numeric for isize { fn zero() -> Self { 0 } }

impl Numeric for f32 { fn zero() -> Self { 0.0 } }
impl Floaty for f32 { }
impl Numeric for f64 { fn zero() -> Self { 0.0 } }
impl Floaty for f64 { }

// use the above traits to constrain T, so we can do all the checks
fn divide<T>(a: T, b: T) -> Option<T>
where
    T: Numeric + PartialEq + Div<Output=T>
{
    if b == T::zero() {
        None
    } else {
        Some(a / b)
    }
}
// uses the Floaty marker trait to force only the use of floating point numbers
fn accept_only_floats<T: Floaty>(f: T) -> T { f }

println!("{:?}", divide(5, 2));
// println!("{:?}", divide(5.0, 2)); // ERROR: f64 != i32
accept_only_floats(2.0f32);
// accept_only_floats(2u32); // ERROR
```

### Common traits in std
- std::ops::{Add, Mul, Div, Sub}
- std::marker::{Sized, Sync, Send}
- std::default::Default
- std::{clone::Clone, marker::Copy}
- Into/From
- AsRef/AsMut
- std::ops::Drop

## Lifetime annotations
```rust
fn return_first<'a, 'b>(a: &'a str, b: &'b str) -> &'a str { a }
fn return_static() -> &'static str { "hello world" }
fn return_self(&self) -> &Self { self }
// fn return_self<'a>(&'a self) -> &'a Self { self } // previous expanded
// add constraints on lifetimes
fn a_longer_than_b<'a: 'b, 'b>(a: &'a str, b: &'b str) -> &'a str { a }

// higher kinded lifetimes
trait TakesRef<'a, T> {
    // return something with a lifetime on the trait level
    fn foo() -> &'a T;
}
// make T generic for all lifetimes 'a
fn use_lifetime<T>()
where
    T: for<'a> TakesRef<'a, u32>,
{
    println!("{}", T::foo());
}
```

## Pattern matching
Patterns that can be commonly used in a match:

Pattern | Meaning
-|-
`0..10` | Range from 0 to 10 (exclusive)
`0..=10` | Range from 0 to 10 (inclusive)
`0 \| 1` | 0 or 1
`Some(value)` | Destructuring of Some and capturing value
`Some(Some(value))` | Destructuring can be nested
`[a, b, c, ..]` | Destructuring first 3 elements and allow more
`(a, b, c, ..)` | Destructuring tuple
`MyStruct { age, name, .. }` | Destructuring struct and allow for more fields