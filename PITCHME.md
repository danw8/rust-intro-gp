## Rust
[rust-lang.org]

---

#### What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

---

#### Who uses Rust?
* Mozilla (Servo/Firefox) they are also the creator of Rust.
* Microsoft (vscode)
* Dropbox
* Coursera
* npm

---

#### Who cares?
Let's look at some code

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

### Modules

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
        ...
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

#### Error Handling

panic! vs Result
* panic! is for unrecoverable errors
* Result is for error that might be recoverable

---
panic! example
```
fn main() {
	panic!("crash and burn");
}
```

```
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

---

```
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

---

Result example
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
#### Generics

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
Now we want to check the largest for an array of f64

---

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

@[1](The **T** is the type we send into the funciton largest)
@[1]( **PartialOrd** and **Copy** are Traits)
@[1]( The **+** between the traits means that T must implement both these traits)
@[1-9]( Rusts numeric types(i32, f64, u8 etc...) already implement both these traits so all of the numeric types can now be used in this funciton.)

---



