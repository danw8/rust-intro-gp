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

### Modules

* The '**mod**' keyword declares a new module
* Module functions, types and constants are private by default, '**pub**' makes them visible
* The '**use**' keyword brings modules or definitions in modules into scope

---

#### Modules in multiple files

* The file or folder name matches the module name
* If a folder is used it needs a mod.rs file inside it.

---

#### Examples

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




