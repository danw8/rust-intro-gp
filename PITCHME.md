## Rust
[rust-lang.org](rust-lang.org)

---

#### Disclaimer

I am an enthusiast, not an expert.

---

#### What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

---

#### What is Rust good at?

* Kernels/operating systems
* Device drivers
* Web applications
* Games
* Etc...

---

#### Who uses Rust?
* Mozilla (Servo/Firefox) they are also the creators of Rust.
* Microsoft (ripgrep in vscode) 
* Dropbox
* Coursera
* npm

---

#### Use Rust if

You want
* Fast, zero cost abstractions
* No Garbage collector
* Memory safety at compile time
* Help preventing programmer bugs
* Modern syntax
* Low level results from a high level language
* Interoperability with C (Can create C libraries)

--- 

#### Don't use Rust if

You have
* Large existing code base written in language X
* Developers only willing to work in language X
* A Company that dictates use of language X
* A love for Garbage collection
* A hatred for thinking about memory

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

#### Tests

* rustc --test test.rs
* run the resulting executable

---

#### Using Cargo 

* cargo new hello_world --bin
* cd hello_world
* cargo build
* cargo test

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

* f64 a 64 bit floating point number (default)
* f32 a 32 bit floating point number

```
let x: f64 = 5.0;
```

---

#### Strings

* **&str** a pointer to a borrowed string. 
* **String** a owned struct that contains and helps with string manipulation.

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

#### Functions

```
fn add_two(num: i32) -> i32 {
	num + 2
}

fn main() {
	let four = add_two(2);
	println!("{}", four);
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

#### Result unwrap
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
@[4](Unwrap will panic if the result is Err)

---

#### Option

```
pub enum Option<T> {
    None,
    Some(T),
}
```

@[1-4](Options are similar to results)
@[1-4](Options don't have an error, they have None)
@[1-4](Options panic when unwrap is called and the value is None)
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
@[1-9]( Rusts numeric types i32, f64, u8 etc already implement both these traits so all of the numeric types can now be used in this funciton.)

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
for count in counter.into_iter() {
	println!("The count is: {}", count);
}
```

---

#### Iterator functions

```
let counter = Counter { count: 0 };
let count_sum = counter.sum(); // 15

let counter = Counter { count: 0 };
let count_map : Vec<u32> = counter.into_iter().map(|x| x * 2).collect(); // [2, 4, 6, 8, 10]

let counter = Counter { count: 0 };
let count_filter : Vec<u32> = counter.into_iter().filter(|x| x % 2 == 0).collect(); // [2, 4]
```

---

### Ownership

---

#### Copy

```
let s1 = "hi";
let s2 = s1;
println!("{}{}", s1, s1);
```

---

#### Move

```
let s1 = String::from("hi");
let s2 = s1;
println!("{}", s2); // This is okay
println!("{}{}", s1, s2); // value used here after move 
```

@[1](String is not Copy)
@[2](The value of s1 is moved into s2 here, s2 now owns the value)
@[3]( The value can be used from s2)
@[4](It is an error to use a value after it is moved.)

---

#### Clone

```
let s1 = String::from("hi");
let s2 = s1.clone();
println!("{}{}", s1, s2);
```

@[1](String is Clone)
@[2](The value of s1 is cloned)
@[2](s1 and s2 are now different memory on the heap that have the same value)
@[3](We can use both s1 and s2 again)

---

* Copy - bitcopy of data on the stack
  * used with type that don't own other elements on the heap
  * i32, f64, &str etc...
* Move - changes what pointer is pointing to the heap memory
  * used with heap allocated types
  * String, Vec, HashMap etc...
* Clone - The memory duplicated on the heap to a new variable
  * used with heap allocated types
  * String, Vec, Hasmap etc...

---

#### Function that borrows

```
fn print_borrowed_string(on_loan: &String) {
	println!("I'm borrowing the {}", on_loan);
}
```

---

#### Function that owns

```
fn print_owned_string(mine_now: String) {
	println!("The {} is mine now", mine_now);
}
```

---

#### Function that returns ownership

```
fn print_owned_string_but_give_it_back(mine: String) -> String{
	println!("{}", mine);
	mine // nevermind you can have it back
}
```

---

### Smart Pointers

---

#### Box<T>

```
let b = Box::new(5);
println!("b = {}", b);
```

@[1](A pointer to the value 5 on the heap)

---

#### Rc<T> Reference Counting

```
use std::rc::Rc;

let a = Rc::new(5);
let b = a.clone();
```

@[3](A reference counted pointer to a value on the heap)
@[4](cloning a Rc increases the reference count.)
@[3-4](The data is shared but not mutable)

---

#### Rc<RefCell<T>>
#### Shareable mutable container

```
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(Vec::<String>::new()));
    let another_way = shared_map.clone();
    shared_map.borrow_mut().push(String::from("Inserted from shared_map"));
    println!("{:?}", *another_way);
    another_way.borrow_mut().push(String::from("Inserted from another_way"));
    println!("{:?}", *shared_map);
}
```

@[1-11](Data is shared and mutable)
@[1-11](Rc<RefCell<T>> is good for single threaded sharing)

---

#### Arc<T> Atomic Reference Counting

```
use std::sync::Arc;
use std::thread;

fn main() {
    let shared_data = Arc::new(5);
    
    let val = Arc::clone(&shared_data);
    let child = thread::spawn(move || {
        for i in 1..5 {
            let value_plus_i = *val + i;
            println!("val plus i : {}", value_plus_i);
        }
    });
    
    for x in 1..5 {
        let value_plus_x = *shared_data + x;
        println!("val plus x : {}", value_plus_x);
    }

    let _ = child.join();
}
```

@[5-18]( Arc<T> is great for sharing immutable data accross threads)

---

#### Arc<Mutex<T>>
#### Thread safe lock

```
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(10);
    let shared_data = Arc::new(Mutex::new(5));
    
    let shared_data2 = Arc::clone(&shared_data);
    let child = thread::spawn(move || {
        for _ in 1..5 {
            {
                let mut data = shared_data2.lock().unwrap();
                *data += 1;
                println!("data updated in child: {:?}", *data);
            } // data unlocks here
            thread::sleep(ten_millis);
        }
    });
    
    for _ in 1..5 {
        {
            let mut data = shared_data.lock().unwrap();
            *data += 1;
            println!("data updated in main: {:?}", *data);
        } // data unlocks here
        thread::sleep(ten_millis);
    }
    
    let _ = child.join();
}
```

---

#### Web Development

* Rocket - Backend web framework
* Quasar - Frontend Web Framework(emscripten)
* Maud - Compile time template library
* Diesel - An Object-relational Mapper(ORM) for rust

---

#### More Cool libraries

* Vulkano - Safe wrapper for vulkan development
* gfx-rs - Graphics library
* tokio - A futures based async I/O library
* piston - A modular Game engine
* gtk-rs - Rust bindings to GLib, GDK3, GTK+ 3 and Cairo

---

#### Tools

* Editors - vscode, atom, visual studio, etc.
* IDE - RustDt, Intellij(Rust extension)
* Rust Language Server
* Racer

---

#### Install rust Or
Install
[rust-lang.org](rust-lang.org)

Play
[https://play.rust-lang.org/](https://play.rust-lang.org/)

----

#### Rustlings

[https://github.com/carols10cents/rustlings](https://github.com/carols10cents/rustlings)