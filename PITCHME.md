## Rust
[rust-lang.org](rust-lang.org)

---

#### What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

---

#### Who uses Rust?
* Mozilla (Servo/Firefox) they are also the creators of Rust.
* Microsoft (ripgrep in vscode) 
* Dropbox
* Coursera
* npm

---

#### Rust vs ?

Why use Rust?
* Fast, zero cost abstractions
* No Garbage collector
* Memory safety at compile time
* Designed to help prevent programmer bugs
* Modern syntax
* Low level results from a high level language
* Interoperability with C (Can create C libraries)

--- 

#### Rust vs ?

Not use Rust
* Needs more mature libraries
* Large existing code base written in language X
* All of our developers are only willing to work in language X

---

### Show me the code

---

#### Traditional Hello World

```
fn main() {
	println!("Hello World!");
}
```

---

#### Compiling Hello World

* rustc main.rs

---

#### Using Cargo 

* cargo new hello_world --bin
* cd hello_world
* cargo build

---
#### Variable binding

```
let i: i32 = 4;
let x = 5;
```

@[1](Use the **let** keyword.)
@[1](The **: i32** is the explicate type binding)
@[2](Uses implicate bindings also)

---

### Types

---

#### Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

```
let x: i32 = 5;
```

---

#### Floats

* f64 a 64 bit decimal (default)
* f32 a 32 bit decimal

```
let x: f64 = 5.0;
```

---

#### Strings

* **&str** a pointer to a borrowed string. 
* **String** a struct that contains and helps with string manipulation.

```
let name: &str = "Hedgehogs";
let our_name: String = String::from("Hedgehogs");
```

---

#### Boolean

```
let no: bool = false;
```

---

#### Character

```
let a: char = 'a';
```

---

#### Tuples

```
let my_tuple: (char, bool, &str) = ('A', true, "Hedgehog");
let (a, b, c) = my_tuple;
```

---

#### Arrays

```
let a: [i32; 4] = [1, 2, 3, 4];
```

---

#### Structs

```
struct User{
	pub name: String,
	pub email: String,
	pub enabled: bool,
}

impl User {
	fn new(name: String, email: String) -> User {
		User {
			name: name,
			email: email,
			enabled: true,
		}
	}
}
```
@[1-5](Struct data)
@[7-15](Struct implementation)

---

#### Enums

```
enum Bar {
	X(u32),
	Y,
}

impl Bar {
    fn get_x(self) -> Option<u32>{
        match self {
            Bar::X(n) => return Some(n),
            _ => return None,
        }
    }
}

let bar = Bar::X(10);
```
@[2](Enum variants can hold data)
@[6-13](Enums can also be implemented)

---

#### Vectors

```
let numbers: Vec<i32> = Vec::new();
let names: Vec<&str> = vec!["Bob","Stan","Mike"];
```

---

### Control Flow

---

#### If else expressions

```
if x == 5 {
	println!("It's 5!");
} else if x == 6 {
	println!("I love the number 6");
} else {
	println!("It's not 5 or 6");
}
```

---

#### Match Expressions

```
let num: i32 = 6;
let x = match(num){
	5 => "It's five!",
	6 => "I love sixes",
	_ => "It's not 5 or 6",
};
```

---

#### If Let

```
let x = Some(5);
if let Some(five) = x {
	println!("{}", five);
}
```
Best way to match on just one Enum variant

---

#### If assignment

```
let x = 5;
let five  = if x == 5 { "five" } else { "not five" };
```

---

#### loop 

```
loop {
	println!("This is a neverending loop);
}
```

---

#### for loop 

```
let array: [i32; 4] = [1, 2, 3, 4];
for number in array.iter(){
	println!("{}", number);
}
```

---

#### while loop

```
let mut x = 5;
while x > 0 {
	println!("{}", x);
	x = x - 1;
}
```

---

#### named loops

```
fn main() {
	let mut x = 0;
	'outer: loop {
		x += 1;
		println!("In outer loop, x's value is: {}", x);
		'inner: while x >= 3 {
			x += 1;
			println!("In inner loop, x's value is: {}", x);
			if x > 6 { break 'outer }
			if x % 2 == 0 { break 'inner }
		}

	}
}
```

---

### Modules

---

#### Modules

* The '**mod**' keyword declares a new module
* Module functions, types and constants are private by default, '**pub**' makes them visible
* The '**use**' keyword brings modules or definitions in modules into scope

---

#### Modules in multiple files

* The file or folder name matches the module name
* If a folder is used it needs a mod.rs file inside it.

---

#### Module Example

```
mod client{
    pub fn connect(){
        // ...
    }
}
```
This is a module declared in the parents source file

---

```
mod client;
```
* The client module is outside of the parents source
* Rust then looks for a folder named client or a source file named client.rs

---

client.rs

```
pub fn connect(){
    // ...
}
```

---

Directory Structure
```
.
+-- src  
|  +-- main.rs  
|  |  +-- client  
|  |  |  +-- mod.rs  
```

mod.rs
```
pub fn connect(){
    // ...
}
```

---

### Error Handling

---

#### panic! vs Result

* panic! is for unrecoverable errors
* Result is for error that might be recoverable

---

#### panic! example

```
fn main() {
	panic!("crash and burn");
}
```

ouput  
  
thread 'main' panicked at 'crash and burn', src/main.rs:2  
note: Run with `RUST_BACKTRACE=1` for a backtrace.  
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)  

---

#### Result Enum

```
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

---

#### Result example
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

---

### Generics

---

#### Non-Generic Function

```
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```
@[1](This is good for only i32, Now we want to check the largest for an array of f64)

---

#### Generic Function

```
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

@[1](The **T** is the type we send into the largest function )
@[1]( **PartialOrd** and **Copy** are Traits)
@[1]( The **+** between the traits means that T must implement both these traits)
@[1-9]( Rusts numeric types(i32, f64, u8 etc.) already implement both these traits so all of the numeric types can now be used in this funciton.)

---

### Implementing traits

---

#### Defining Traits

```
trait HasArea {
	fn area(&self) -> f64;
}
```

---

#### Implement HasArea for Circle

```
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

---

#### Implement HasArea for Square

```
struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
```
@[1-11](Implementing the HasArea trait for the Square struct)
@[1-11](Traits can be implemented for any type including basic types)
@[1-11](It is generally bad practise to implement for basic types though)

---

#### Using HasArea With Circle and Square

```
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
    let c = Circle { x: 0.0f64, y: 0.0f64, radius: 1.0f64, };
    let s = Square { x: 0.0f64, y: 0.0f64, side: 1.0f64, };
    print_area(c);
    print_area(s);
}
```

---

### Lifetimes

* Lifetimes can be thought of similar to the scope of a variable
* You can give lifetimes a name
* Many lifetimes can be inferred often called Lifetime Elision
* Lifetimes prevent dangling references

---
#### The Borrow Checker

```
{
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    println!("r: {}", r); // |
                   //        |
                   // -------+
}
```

This code gets a compilation error of `error: 'x' does not live long enough`

---

#### Explicate lifetime annotation(function)

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

@[1](The x: **&'a** str, y: **&'a str** are input lifetimes)
@[1](The **&'a** str return type is the output lifetime)
@[1](The **<&'a>** at the begining of the function is the lifetime definition)

---

#### Explicate lifetime annotation(struct)

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

@[1](Lifetimes on struct cannot be elided)

---

#### Lifetimes on impl required
```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

---

#### Example of Type Parameters, Trait Bounds and Lifetimes together
```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

### Functional Language Features

---

#### Closures

```
fn  add_one_v1   (x: i32) -> i32 { x + 1 }
let add_one_v2 = |x: i32| -> i32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

@[1](function)
@[2-4](closures)

---

#### Iterators

```
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

---

#### Impl an Iterator
 
```
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

#### Using Counter for loop

```
let counter = Counter { count: 0 };
for count in counter.iter() {
	println!("The count is: {}" count);
}
```

#### Iterator functions

```
let counter = Counter { count: 0 };

```